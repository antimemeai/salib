#!/usr/bin/env bash
# Run every verify/sage script: assertion scripts check derivations, generator
# scripts rewrite their committed oracles under crates/*/tests/oracles/. Fails
# if any script fails or a regenerated oracle differs from the committed one.
# Local and neuroses only; CI runs the committed oracles through cargo test.
set -euo pipefail
cd "$(dirname "$0")/.."
export SALIB_ROOT="$PWD"
export CROLIB="${CROLIB:-$PWD/../crolib}"
SAGE="${SAGE:-/opt/homebrew/anaconda3/envs/sage/bin/sage}"
[ -x "$SAGE" ] || { echo "sage not found at $SAGE; set SAGE (see verify/sage/environment.yml)" >&2; exit 1; }
if grep -q 'balanced.sage' verify/sage/*.sage 2>/dev/null; then
    [ -f "$CROLIB/verify/sage/balanced.sage" ] || { echo "crolib Sage library not found; set CROLIB" >&2; exit 1; }
fi
SAGE_PYTHON="${SAGE_PYTHON:-$(dirname "$SAGE")/python}"
# .sage scripts are preparsed by Sage (symbolic work); .py scripts run under
# the Sage environment's python (exact or high-precision numerics).
for script in verify/sage/*.sage verify/sage/*.py; do
    [ -e "$script" ] || continue
    echo "== $script"
    case "$script" in
        *.sage) "$SAGE" "$script" ;;
        *.py) "$SAGE_PYTHON" -I "$script" ;;
    esac
done
# Compare against HEAD, including untracked oracle files.
oracles=':(glob)crates/*/tests/oracles/**'
if [ -n "$(git status --porcelain --untracked-files=all -- "$oracles")" ]; then
    git status --short --untracked-files=all -- "$oracles" >&2
    echo "generated oracles differ from HEAD; review and commit" >&2
    exit 1
fi
