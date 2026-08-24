#!/usr/bin/env bash
# Added by the grok-build-hardened project.

set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck disable=SC1091
source "${script_dir}/common.sh"

hardening_require_command git
hardening_require_command date
hardening_require_clean_tree

branch="$(hardening_current_branch)"
[[ "$branch" == main ]] || hardening_fail "upstream rebase must run on main, not ${branch}"

upstream_url="$(git -C "$REPO_ROOT" remote get-url upstream 2>/dev/null || true)"
case "$upstream_url" in
    https://github.com/xai-org/grok-build.git|git@github.com:xai-org/grok-build.git|ssh://git@github.com/xai-org/grok-build.git) ;;
    *) hardening_fail "remote 'upstream' is missing or points to an unexpected repository" ;;
esac

git -C "$REPO_ROOT" fetch upstream "$UPSTREAM_BRANCH" --tags
target_ref="${1:-upstream/${UPSTREAM_BRANCH}}"
target_commit="$(git -C "$REPO_ROOT" rev-parse --verify "${target_ref}^{commit}")"

git -C "$REPO_ROOT" merge-base --is-ancestor "$UPSTREAM_COMMIT" HEAD \
    || hardening_fail "current branch is not based on approved commit ${UPSTREAM_COMMIT}"
git -C "$REPO_ROOT" merge-base --is-ancestor "$UPSTREAM_COMMIT" "$target_commit" \
    || hardening_fail "target ${target_commit} is not a descendant of the approved base"

if [[ "$target_commit" == "$UPSTREAM_COMMIT" ]]; then
    printf 'REBASE NOT NEEDED: upstream remains at %s\n' "$UPSTREAM_COMMIT"
    exit 0
fi

"${script_dir}/upstream-status.sh" "$target_commit"

backup_tag="backup/pre-rebase-$(date -u +%Y%m%dT%H%M%SZ)-$(git -C "$REPO_ROOT" rev-parse --short=12 HEAD)"
git -C "$REPO_ROOT" tag -a "$backup_tag" -m "Backup before rebase from ${UPSTREAM_COMMIT} to ${target_commit}"
printf 'Created local backup tag: %s\n' "$backup_tag"

if ! git -C "$REPO_ROOT" rebase --onto "$target_commit" "$UPSTREAM_COMMIT" "$branch"; then
    printf 'Rebase stopped. Resolve each conflict, then run git rebase --continue.\n' >&2
    printf 'To abandon it, run git rebase --abort; backup: %s\n' "$backup_tag" >&2
    exit 1
fi

printf 'REBASE COMPLETE, BUT NOT APPROVED.\n'
printf 'The hardening check must fail until a manual source audit updates:\n'
printf '  .hardened/upstream.env\n'
printf '  .hardened/source-paths.tsv\n'
printf 'Follow UPSTREAM.md before committing or tagging a release.\n'
