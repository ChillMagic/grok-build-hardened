#!/usr/bin/env bash
# Added by the grok-build-hardened project.

set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck disable=SC1091
source "${script_dir}/common.sh"

for command_name in cargo protoc rg; do
    hardening_require_command "$command_name"
done

"${script_dir}/check.sh"

target_dir="${GROK_HARDENED_TARGET_DIR:-${REPO_ROOT}/target/hardened}"

env CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$target_dir" \
    cargo fmt --manifest-path "${REPO_ROOT}/Cargo.toml" --all -- --check
env CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$target_dir" \
    cargo build --manifest-path "${REPO_ROOT}/Cargo.toml" \
        --locked --offline --release -p xai-grok-pager-bin

binary="${target_dir}/release/xai-grok-pager"
[[ -x "$binary" ]] || hardening_fail "release binary was not produced: ${binary}"

version_output="$($binary --version)"
[[ "$version_output" == *'[privacy]'* ]] \
    || hardening_fail "binary lacks the privacy version marker"

runtime_must_be_blocked() {
    local output
    if output="$($binary "$@" 2>&1)"; then
        hardening_fail "command unexpectedly succeeded: grok $*"
    fi
    case "$output" in
        *disabled*|*removed*) ;;
        *) hardening_fail "blocked command did not return the privacy marker: grok $*" ;;
    esac
}

runtime_must_be_blocked update --check
runtime_must_be_blocked update
runtime_must_be_blocked setup
runtime_must_be_blocked workspace status
runtime_must_be_blocked share privacy-audit-session
runtime_must_be_blocked trace privacy-audit-session

printf 'OFFLINE BUILD OK: %s\n' "$binary"
printf '  %s\n' "$version_output"
