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
[ -f "$CROLIB/verify/sage/balanced.sage" ] || { echo "crolib Sage library not found; set CROLIB" >&2; exit 1; }
for script in verify/sage/*.sage; do
    echo "== $script"
    "$SAGE" "$script"
done
git diff --exit-code -- 'crates/*/tests/oracles/' || { echo "generated oracles changed; review and commit" >&2; exit 1; }
