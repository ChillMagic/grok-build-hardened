#!/usr/bin/env bash
# Added by the grok-build-hardened project.

set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck disable=SC1091
source "${script_dir}/common.sh"

for command_name in git sort comm cut grep sed mktemp wc tr; do
    hardening_require_command "$command_name"
done

target_ref="${1:-upstream/${UPSTREAM_BRANCH}}"
git -C "$REPO_ROOT" rev-parse --verify --quiet "${target_ref}^{commit}" >/dev/null \
    || hardening_fail "unknown target ref: ${target_ref}; run git fetch upstream ${UPSTREAM_BRANCH} first"
target_commit="$(git -C "$REPO_ROOT" rev-parse "${target_ref}^{commit}")"

printf 'Approved upstream: %s (%s)\n' "$UPSTREAM_COMMIT" "$UPSTREAM_VERSION"
printf 'Available target:  %s (%s)\n' "$target_commit" "$target_ref"

if [[ "$target_commit" == "$UPSTREAM_COMMIT" ]]; then
    printf 'UPSTREAM STATUS OK: already at the approved upstream commit\n'
    exit 0
fi

git -C "$REPO_ROOT" merge-base --is-ancestor "$UPSTREAM_COMMIT" "$target_commit" \
    || hardening_fail "target is not a descendant of the approved upstream commit"

temporary_dir="$(mktemp -d "${TMPDIR:-/tmp}/grok-upstream-status.XXXXXX")"
cleanup() {
    rm -f -- "${temporary_dir}/upstream.txt" "${temporary_dir}/hardened.txt" \
        "${temporary_dir}/overlap.txt" "${temporary_dir}/risk.txt"
    rmdir "$temporary_dir" 2>/dev/null || true
}
trap cleanup EXIT

git -C "$REPO_ROOT" diff --name-only "$UPSTREAM_COMMIT".."$target_commit" \
    | sort -u >"${temporary_dir}/upstream.txt"
tail -n +2 "${REPO_ROOT}/.hardened/source-paths.tsv" \
    | cut -f2 | sort -u >"${temporary_dir}/hardened.txt"
comm -12 "${temporary_dir}/upstream.txt" "${temporary_dir}/hardened.txt" \
    >"${temporary_dir}/overlap.txt"
grep -Ei \
    '(upload|telemetry|remote|managed|storage|feedback|share|update|policy|sentry|mixpanel|workspace|archive|trace|bundle|hook|http|grpc|websocket)' \
    "${temporary_dir}/upstream.txt" >"${temporary_dir}/risk.txt" || true

changed_count="$(wc -l <"${temporary_dir}/upstream.txt" | tr -d '[:space:]')"
overlap_count="$(wc -l <"${temporary_dir}/overlap.txt" | tr -d '[:space:]')"
risk_count="$(wc -l <"${temporary_dir}/risk.txt" | tr -d '[:space:]')"

printf '\nUpstream changed paths: %s\n' "$changed_count"
printf 'Overlap with hardening paths: %s\n' "$overlap_count"
if [[ "$overlap_count" != 0 ]]; then
    sed 's/^/  OVERLAP  /' "${temporary_dir}/overlap.txt"
fi
printf '\nSecurity-relevant path names: %s\n' "$risk_count"
if [[ "$risk_count" != 0 ]]; then
    sed 's/^/  REVIEW   /' "${temporary_dir}/risk.txt"
fi

printf '\nThis report is triage only; it does not approve the new upstream source.\n'
