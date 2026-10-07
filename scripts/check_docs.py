#!/usr/bin/env python3
"""Check public Markdown links, citation destinations, fences, and release examples.

Uses only Python's standard library. --online also GETs the recorded literature
URLs; a successful HTTP request proves reachability, not support for a claim.
Claim-level evidence and numerical oracles are in papers/2026-10-07-literature-audit.md.
"""
import argparse
import concurrent.futures
import json
from pathlib import Path
import re
import sys
import urllib.error
import urllib.parse
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
PAGES = [ROOT / 'README.md', *sorted(ROOT.glob('crates/*/README.md'))]
PAGES += [ROOT / 'docs' / name for name in (
    'index.md', 'choosing.md', 'crates.md', 'internals.md', 'quickstart.md',
    'benchmarks.md', 'bibliography.md', 'release-0.3.0.md')]
PAGES += sorted(ROOT.glob('docs/methods/*.md'))
LINK = re.compile(r'\[[^\]\n]*(?:\][^\]\n]*)?\]\(([^\s)]+)\)')


def anchors(text):
    result = set(re.findall(r'<a\s+id="([^"]+)"', text))
    counts = {}
    for title in re.findall(r'^#{1,6}\s+(.+?)\s*#*$', text, re.M):
        slug = re.sub(r'[^\w\- ]', '', title.lower()).replace(' ', '-')
        count = counts.get(slug, 0)
        counts[slug] = count + 1
        result.add(slug if count == 0 else f'{slug}-{count}')
    return result


def outside_code(text):
    # These pages use backtick fences. Keep link checks out of executable examples.
    return re.sub(r'^```[^\n]*\n.*?^```\s*$', '', text, flags=re.M | re.S)


def check_local():
    errors = []
    version = re.search(r'^version = "([^"]+)"',
                        (ROOT / 'crates/salib/Cargo.toml').read_text(), re.M)[1]
    major_minor = '.'.join(version.split('.')[:2])
    for page in PAGES:
        text = page.read_text()
        label = str(page.relative_to(ROOT))
        if len(re.findall(r'^```', text, re.M)) % 2:
            errors.append(f'{label}: unclosed code fence')
        for match in re.finditer(r'salib(?:-[a-z]+)?\s*=\s*(?:\{\s*version\s*=\s*)?"([^"]+)"', text):
            if match[1] not in (version, major_minor):
                errors.append(f'{label}: dependency example selects {match[1]}, expected {major_minor}')
        for match in LINK.finditer(outside_code(text)):
            target = urllib.parse.urlsplit(match[1])
            if target.scheme or target.netloc:
                # Validate this repository's GitHub document links locally too.
                prefix = 'https://github.com/antimemeai/salib/blob/master/'
                if match[1].startswith(prefix):
                    target = urllib.parse.urlsplit(match[1][len(prefix):])
                    destination = ROOT / urllib.parse.unquote(target.path)
                else:
                    continue
            else:
                destination = page.parent / urllib.parse.unquote(target.path) if target.path else page
            if not destination.exists():
                errors.append(f'{label}: missing link target {match[1]}')
            elif target.fragment and destination.is_file() and destination.suffix == '.md':
                if urllib.parse.unquote(target.fragment) not in anchors(destination.read_text()):
                    errors.append(f'{label}: missing anchor {match[1]}')
    sources = json.loads((ROOT / 'papers/literature-sources.json').read_text())['sources']
    bibliography = (ROOT / 'docs/bibliography.md').read_text()
    ids = set()
    for source in sources:
        key, url = source['id'], source['url']
        if key in ids:
            errors.append(f'duplicate citation ID: {key}')
        ids.add(key)
        marker = f'<a id="{key}"></a>'
        if marker not in bibliography:
            errors.append(f'missing bibliography ID: {key}')
            continue
        # Alias anchors precede the named entry; split at the next blank-separated entry.
        entry = bibliography.split(marker, 1)[1].split('\n\n', 1)[0]
        if f'({url})' not in entry:
            errors.append(f'bibliography source differs from registry: {key}')
    for page in sorted((ROOT / 'docs/methods').glob('*.md')):
        text = outside_code(page.read_text())
        if not any(f'({s["url"]})' in text for s in sources):
            errors.append(f'{page.relative_to(ROOT)}: no direct registered literature citation')
    return errors, sources


def reachable(source):
    req = urllib.request.Request(source['url'], headers={'User-Agent': 'salib-doc-check/0.3'})
    try:
        with urllib.request.urlopen(req, timeout=15) as response:
            response.read(1024)
            return source['id'], response.status, None
    except (OSError, ValueError, urllib.error.URLError) as error:
        return source['id'], None, str(error)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--online', action='store_true')
    args = parser.parse_args()
    errors, sources = check_local()
    for error in errors:
        print('FAIL:', error)
    if not errors:
        print(f'PASS: {len(PAGES)} public pages; release examples, local links, fences, '
              f'and {len(sources)} registered citation destinations.')
    if args.online:
        # Keep concurrency low; publisher and DOI services may rate-limit requests.
        with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
            for key, status, error in pool.map(reachable, sources):
                if error:
                    errors.append(f'{key}: {error}')
                    print(f'UNVERIFIED HTTP: {key}: {error}', flush=True)
                else:
                    print(f'HTTP {status}: {key}', flush=True)
        print('HTTP results check access only. Full-text evidence is recorded in the audit reports.')
    return int(bool(errors))


if __name__ == '__main__':
    sys.exit(main())
