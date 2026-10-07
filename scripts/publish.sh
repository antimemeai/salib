#!/usr/bin/env bash
set -euo pipefail

# Package and verify this workspace's existing release versions using Cargo.
# No version bump or cargo-workspaces dependency. Multi-package publishing
# requires multi-package publishing support (verified with Cargo 1.95);
# consumers retain the documented feature-specific Rust requirements.
# Usage: ./scripts/publish.sh [--dry-run|--publish]
# Default: dry run. --publish uploads and requires a clean Git working tree.

cd "$(dirname "$0")/.."
mode="${1:---dry-run}"
if [[ $# -gt 1 ]]; then
    echo "Usage: $0 [--dry-run|--publish]" >&2
    exit 2
fi
case "$mode" in
    --dry-run)
        cargo publish --dry-run --workspace --exclude salib-models --locked --allow-dirty --features salib/full,salib/serde
        ;;
    --publish)
        if [[ -n "$(git status --porcelain)" ]]; then
            echo "Commit the reviewed release changes before publishing." >&2
            exit 1
        fi
        cargo publish --workspace --exclude salib-models --locked --features salib/full,salib/serde
        ;;
    *)
        echo "Usage: $0 [--dry-run|--publish]" >&2
        exit 2
        ;;
esac
