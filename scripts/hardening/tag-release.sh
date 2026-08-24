#!/usr/bin/env bash
# Added by the grok-build-hardened project.

set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck disable=SC1091
source "${script_dir}/common.sh"

hardening_require_command git
hardening_require_clean_tree

branch="$(hardening_current_branch)"
[[ "$branch" == main ]] || hardening_fail "release tags may be created only from main"

expected_tag="$(hardening_release_tag)"
requested_tag="${1:-$expected_tag}"
[[ "$requested_tag" == "$expected_tag" ]] \
    || hardening_fail "expected tag ${expected_tag}, received ${requested_tag}"

if git -C "$REPO_ROOT" rev-parse --verify --quiet "refs/tags/${requested_tag}" >/dev/null; then
    hardening_fail "tag already exists: ${requested_tag}"
fi

"${script_dir}/check.sh"

git -C "$REPO_ROOT" tag -a "$requested_tag" -m \
    "grok-build-hardened ${requested_tag}; upstream ${UPSTREAM_COMMIT}"

printf 'TAG CREATED: %s\n' "$requested_tag"
printf 'Review it with: git show --stat %s\n' "$requested_tag"
printf 'Push explicitly with: git push origin %s\n' "$requested_tag"
