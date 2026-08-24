#!/usr/bin/env bash
# Added by the grok-build-hardened project.

set -euo pipefail

readonly HARDENING_SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
readonly REPO_ROOT="$(cd -- "${HARDENING_SCRIPT_DIR}/../.." && pwd -P)"
readonly HARDENING_METADATA="${REPO_ROOT}/.hardened/upstream.env"

hardening_fail() {
    printf 'HARDENING CHECK FAILED: %s\n' "$*" >&2
    exit 1
}

hardening_require_command() {
    command -v "$1" >/dev/null 2>&1 \
        || hardening_fail "missing required command: $1"
}

hardening_require_clean_tree() {
    [[ -z "$(git -C "$REPO_ROOT" status --porcelain --untracked-files=all)" ]] \
        || hardening_fail "the Git worktree must be clean"
}

hardening_current_branch() {
    git -C "$REPO_ROOT" symbolic-ref --quiet --short HEAD \
        || hardening_fail "detached HEAD is not allowed for this operation"
}

git -C "$REPO_ROOT" rev-parse --git-dir >/dev/null 2>&1 \
    || hardening_fail "repository metadata is missing: ${REPO_ROOT}"
[[ -f "$HARDENING_METADATA" ]] \
    || hardening_fail "missing ${HARDENING_METADATA}"

# This file is tracked by the repository and validated by check.sh.
# shellcheck disable=SC1090
source "$HARDENING_METADATA"

: "${UPSTREAM_REPOSITORY:?missing UPSTREAM_REPOSITORY}"
: "${UPSTREAM_BRANCH:?missing UPSTREAM_BRANCH}"
: "${UPSTREAM_COMMIT:?missing UPSTREAM_COMMIT}"
: "${UPSTREAM_VERSION:?missing UPSTREAM_VERSION}"
: "${HARDENED_REVISION:?missing HARDENED_REVISION}"

hardening_release_tag() {
    if [[ "$HARDENED_REVISION" == 0 ]]; then
        printf 'v%s-hardened\n' "$UPSTREAM_VERSION"
    else
        printf 'v%s-hardened.%s\n' "$UPSTREAM_VERSION" "$HARDENED_REVISION"
    fi
}
