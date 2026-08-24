#!/usr/bin/env bash
# Added by the grok-build-hardened project.

set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck disable=SC1091
source "${script_dir}/common.sh"

hardening_require_command git

origin_url="$(git -C "$REPO_ROOT" remote get-url origin 2>/dev/null || true)"
[[ -n "$origin_url" ]] \
    || hardening_fail "origin is missing; add your fork as origin before continuing"
case "$origin_url" in
    *github.com/xai-org/grok-build.git)
        hardening_fail "origin points to the official repository; rename it to upstream and add your fork as origin"
        ;;
esac

upstream_url="$(git -C "$REPO_ROOT" remote get-url upstream 2>/dev/null || true)"
if [[ -z "$upstream_url" ]]; then
    git -C "$REPO_ROOT" remote add upstream "$UPSTREAM_REPOSITORY"
else
    case "$upstream_url" in
        https://github.com/xai-org/grok-build.git|git@github.com:xai-org/grok-build.git|ssh://git@github.com/xai-org/grok-build.git) ;;
        *) hardening_fail "existing upstream remote points to an unexpected repository: ${upstream_url}" ;;
    esac
fi

git -C "$REPO_ROOT" remote set-url --push upstream DISABLED
git -C "$REPO_ROOT" config remote.pushDefault origin
git -C "$REPO_ROOT" config rerere.enabled true
git -C "$REPO_ROOT" config rerere.autoupdate true

printf 'REMOTE SETUP OK\n'
git -C "$REPO_ROOT" remote -v
