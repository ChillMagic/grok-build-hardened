#!/usr/bin/env bash
# Added by the grok-build-hardened project.

set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck disable=SC1091
source "${script_dir}/common.sh"

for command_name in git grep awk sed cmp diff mktemp head tail; do
    hardening_require_command "$command_name"
done

require_literal() {
    local path="$1"
    local literal="$2"
    grep -Fq -- "$literal" "${REPO_ROOT}/${path}" \
        || hardening_fail "required invariant missing from ${path}: ${literal}"
}

forbid_regex() {
    local pattern="$1"
    shift
    local output
    output="$(grep -ERn -- "$pattern" "$@" 2>/dev/null || true)"
    if [[ -n "$output" ]]; then
        printf '%s\n' "$output" >&2
        hardening_fail "forbidden transport/process primitive found"
    fi
}

[[ "$UPSTREAM_COMMIT" =~ ^[0-9a-f]{40}$ ]] \
    || hardening_fail "UPSTREAM_COMMIT must be a full lowercase Git SHA"
[[ "$UPSTREAM_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+([.-][0-9A-Za-z.-]+)?$ ]] \
    || hardening_fail "invalid UPSTREAM_VERSION: ${UPSTREAM_VERSION}"
[[ "$HARDENED_REVISION" =~ ^[1-9][0-9]*$ ]] \
    || hardening_fail "HARDENED_REVISION must be a positive integer"

git -C "$REPO_ROOT" cat-file -e "${UPSTREAM_COMMIT}^{commit}" 2>/dev/null \
    || hardening_fail "approved upstream commit is not present locally"
git -C "$REPO_ROOT" merge-base --is-ancestor "$UPSTREAM_COMMIT" HEAD \
    || hardening_fail "HEAD is not based on the approved upstream commit"

upstream_url="$(git -C "$REPO_ROOT" remote get-url upstream 2>/dev/null || true)"
case "$upstream_url" in
    https://github.com/xai-org/grok-build.git|git@github.com:xai-org/grok-build.git|ssh://git@github.com/xai-org/grok-build.git) ;;
    *) hardening_fail "remote 'upstream' must point to ${UPSTREAM_REPOSITORY}" ;;
esac

origin_url="$(git -C "$REPO_ROOT" remote get-url --push origin 2>/dev/null || true)"
[[ -n "$origin_url" ]] || hardening_fail "remote 'origin' is missing"
case "$origin_url" in
    *github.com/xai-org/grok-build.git)
        hardening_fail "origin points at the official repository; pushes must target the fork"
        ;;
esac

tracking_ref="refs/remotes/upstream/${UPSTREAM_BRANCH}"
if git -C "$REPO_ROOT" show-ref --verify --quiet "$tracking_ref"; then
    tracking_base="$(git -C "$REPO_ROOT" merge-base HEAD "$tracking_ref")"
    [[ "$tracking_base" == "$UPSTREAM_COMMIT" ]] \
        || hardening_fail "rebased source is not approved: merge-base is ${tracking_base}, metadata names ${UPSTREAM_COMMIT}"
fi

actual_version="$(
    git -C "$REPO_ROOT" show \
        "${UPSTREAM_COMMIT}:crates/codegen/xai-grok-pager-bin/Cargo.toml" \
        | awk -F'"' '/^version[[:space:]]*=/ { print $2; exit }'
)"
[[ "$actual_version" == "$UPSTREAM_VERSION" ]] \
    || hardening_fail "metadata version ${UPSTREAM_VERSION} does not match upstream ${actual_version}"

manifest="${REPO_ROOT}/.hardened/source-paths.tsv"
[[ -f "$manifest" ]] || hardening_fail "missing source path manifest"
temporary_dir="$(mktemp -d "${TMPDIR:-/tmp}/grok-hardened-check.XXXXXX")"
cleanup() {
    rm -f -- "${temporary_dir}/actual.tsv" "${temporary_dir}/expected.tsv" \
        "${temporary_dir}/modified.txt"
    rmdir "$temporary_dir" 2>/dev/null || true
}
trap cleanup EXIT

git -C "$REPO_ROOT" diff --name-status "$UPSTREAM_COMMIT"..HEAD -- \
    Cargo.lock Cargo.toml crates >"${temporary_dir}/actual.tsv"
tail -n +2 "$manifest" >"${temporary_dir}/expected.tsv"
if ! cmp -s "${temporary_dir}/expected.tsv" "${temporary_dir}/actual.tsv"; then
    diff -u "${temporary_dir}/expected.tsv" "${temporary_dir}/actual.tsv" >&2 || true
    hardening_fail "source path manifest differs from the reviewed hardening diff"
fi

git -C "$REPO_ROOT" diff --name-only --diff-filter=M "$UPSTREAM_COMMIT"..HEAD -- \
    Cargo.lock Cargo.toml crates >"${temporary_dir}/modified.txt"
while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    first_line="$(head -n 1 "${REPO_ROOT}/${path}")"
    case "$path" in
        *.rs)
            [[ "$first_line" == '// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.' ]] \
                || hardening_fail "missing Apache 2.0 change notice in ${path}"
            ;;
        *.toml|Cargo.lock)
            [[ "$first_line" == '# Modified by the grok-build-hardened project; see /MODIFICATIONS.md.' ]] \
                || hardening_fail "missing Apache 2.0 change notice in ${path}"
            ;;
        *) hardening_fail "unknown modified source format needs a change-notice rule: ${path}" ;;
    esac
done <"${temporary_dir}/modified.txt"

for document in README.md SECURITY.md CONTRIBUTING.md; do
    first_line="$(head -n 1 "${REPO_ROOT}/${document}")"
    [[ "$first_line" == '<!-- Modified by the grok-build-hardened project; see /MODIFICATIONS.md. -->' ]] \
        || hardening_fail "missing change notice in ${document}"
done

git -C "$REPO_ROOT" diff --quiet "$UPSTREAM_COMMIT"..HEAD -- \
    LICENSE THIRD-PARTY-NOTICES third_party/NOTICE \
    || hardening_fail "upstream license or attribution files were changed"
require_literal "README.md" "This is an unofficial, community-maintained fork"
if grep -Eq 'spacexai-symbol|x\.ai/v1/website' "${REPO_ROOT}/README.md"; then
    hardening_fail "official logo/brand assets must not appear in the fork README"
fi

while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    [[ ! -e "${REPO_ROOT}/${path}" ]] \
        || hardening_fail "removed capability file reappeared: ${path}"
done <<'DELETED_PATHS'
crates/codegen/xai-file-utils/src/circuit_breaker_observer.rs
crates/codegen/xai-file-utils/src/circuit_breaker_observer_tests.rs
crates/codegen/xai-file-utils/src/queue_tests.rs
crates/codegen/xai-file-utils/src/storage_client_breaker_tests.rs
crates/codegen/xai-grok-memory/src/archive.rs
crates/codegen/xai-grok-shell/src/agent/storage_client_tests.rs
crates/codegen/xai-grok-shell/src/managed_config/response.rs
crates/codegen/xai-grok-shell/src/managed_config/tests.rs
crates/codegen/xai-grok-shell/src/remote/client_tests.rs
crates/codegen/xai-grok-shell/src/remote/pull_smoke_test.rs
crates/codegen/xai-grok-telemetry/src/external/emit.rs
crates/codegen/xai-grok-telemetry/src/external/providers.rs
crates/codegen/xai-grok-telemetry/src/external/redact.rs
crates/codegen/xai-grok-telemetry/src/external/tests.rs
crates/codegen/xai-grok-telemetry/src/external/truncate.rs
crates/codegen/xai-grok-telemetry/src/otel_layer/redact.rs
crates/codegen/xai-grok-telemetry/src/otlp_http.rs
crates/codegen/xai-grok-telemetry/tests/external_otlp.rs
crates/codegen/xai-grok-telemetry/tests/external_otlp_gates_on.rs
crates/codegen/xai-grok-telemetry/tests/external_otlp_grpc.rs
crates/codegen/xai-grok-telemetry/tests/external_otlp_grpc_tls.rs
crates/codegen/xai-grok-telemetry/tests/external_otlp_guard.rs
crates/codegen/xai-grok-telemetry/tests/external_otlp_mtls_grpc.rs
crates/codegen/xai-grok-telemetry/tests/external_otlp_mtls_grpc_reject.rs
crates/codegen/xai-grok-telemetry/tests/external_otlp_mtls_http.rs
crates/codegen/xai-grok-telemetry/tests/external_otlp_session_ctx.rs
crates/codegen/xai-grok-telemetry/tests/manual_auth_emit.rs
crates/codegen/xai-grok-telemetry/tests/otlp_collector/mod.rs
crates/codegen/xai-grok-update/src/auto_update_tests.rs
crates/codegen/xai-grok-workspace/src/publish.rs
crates/codegen/xai-mixpanel/Cargo.toml
crates/codegen/xai-mixpanel/src/lib.rs
crates/common/xai-computer-hub-sdk/src/donate_pump.rs
crates/common/xai-computer-hub-sdk/src/log_donate.rs
crates/common/xai-computer-hub-sdk/src/metric_donate.rs
crates/common/xai-computer-hub-sdk/src/trace_donate.rs
DELETED_PATHS

require_literal "crates/codegen/xai-grok-shell/src/privacy_build.rs" "pub const PRIVACY_BUILD: bool = true;"
require_literal "crates/codegen/xai-grok-shell/src/privacy_build.rs" "pub const REMOTE_CONTROL_COMPILED_IN: bool = false;"
require_literal "crates/codegen/xai-grok-shell/src/privacy_build.rs" "pub const PASSIVE_UPLOADS_COMPILED_IN: bool = false;"
require_literal "crates/codegen/xai-grok-update/src/lib.rs" "pub const UPDATER_COMPILED_IN: bool = false;"
require_literal "crates/codegen/xai-grok-telemetry/src/lib.rs" "pub const NETWORK_TELEMETRY_COMPILED_IN: bool = false;"
require_literal "crates/codegen/xai-file-utils/src/lib.rs" "pub const DATA_UPLOADS_COMPILED_IN: bool = false;"
require_literal "crates/codegen/xai-grok-shell/src/remote/client.rs" "pub const REMOTE_BACKEND_COMPILED_IN: bool = false;"
require_literal "crates/codegen/xai-grok-shell/src/managed_config.rs" "pub const MANAGED_CONFIG_COMPILED_IN: bool = false;"
require_literal "crates/codegen/xai-grok-pager-bin/src/main.rs" "fn should_check_for_updates(no_auto_update_flag: bool) -> bool"
require_literal "crates/codegen/xai-grok-pager-bin/src/main.rs" "Command::Mcp(mcp_args)"
require_literal "crates/codegen/xai-grok-pager-bin/src/main.rs" "Command::Plugin(plugin_args)"

for allowed_path in \
    crates/codegen/xai-grok-tools/src/implementations/grok_build/image_gen/mod.rs \
    crates/codegen/xai-grok-tools/src/implementations/grok_build/image_edit/mod.rs \
    crates/codegen/xai-grok-tools/src/implementations/grok_build/video_gen/mod.rs \
    crates/codegen/xai-grok-mcp/src/lib.rs; do
    [[ -f "${REPO_ROOT}/${allowed_path}" ]] \
        || hardening_fail "explicitly retained capability disappeared: ${allowed_path}"
done

forbid_regex \
    'reqwest::Client|hyper::Client|tonic::transport|connect_async|TcpStream|UdpSocket|Command::new|std::process::Command|tokio::process' \
    "${REPO_ROOT}/crates/codegen/xai-grok-telemetry/src/client.rs" \
    "${REPO_ROOT}/crates/codegen/xai-grok-telemetry/src/config.rs" \
    "${REPO_ROOT}/crates/codegen/xai-grok-telemetry/src/external" \
    "${REPO_ROOT}/crates/codegen/xai-grok-telemetry/src/otel_layer/mod.rs" \
    "${REPO_ROOT}/crates/codegen/xai-grok-update/src" \
    "${REPO_ROOT}/crates/codegen/xai-file-utils/src/gcs.rs" \
    "${REPO_ROOT}/crates/codegen/xai-file-utils/src/s3.rs" \
    "${REPO_ROOT}/crates/codegen/xai-file-utils/src/storage_client.rs" \
    "${REPO_ROOT}/crates/codegen/xai-file-utils/src/queue.rs" \
    "${REPO_ROOT}/crates/codegen/xai-grok-bundle/src/lib.rs" \
    "${REPO_ROOT}/crates/codegen/xai-grok-hooks/src/runner/http.rs" \
    "${REPO_ROOT}/crates/codegen/xai-grok-shell/src/managed_config.rs" \
    "${REPO_ROOT}/crates/codegen/xai-grok-shell/src/remote" \
    "${REPO_ROOT}/crates/codegen/xai-grok-workspace/src/export_github.rs" \
    "${REPO_ROOT}/crates/codegen/xai-grok-memory/src/embedding.rs" \
    "${REPO_ROOT}/crates/codegen/xai-grok-shell-base/src/util/changelog.rs"

forbid_regex \
    'std::env|https?://|OTEL_EXPORTER_|GROK_TELEMETRY_BUILD_' \
    "${REPO_ROOT}/crates/codegen/xai-grok-telemetry/src/config.rs" \
    "${REPO_ROOT}/crates/codegen/xai-grok-telemetry/src/external/config.rs"

forbid_regex \
    'https?://|reqwest|ureq|curl|wget|Command::new|std::process::Command' \
    "${REPO_ROOT}/crates/codegen/xai-grok-tools/build.rs" \
    "${REPO_ROOT}/crates/codegen/xai-grok-shell/build.rs"

added_lines="$(git -C "$REPO_ROOT" diff --unified=0 "$UPSTREAM_COMMIT"..HEAD -- . ':!Cargo.lock' | grep '^+' || true)"
if printf '%s\n' "$added_lines" | grep -Eq \
    '(-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----|gh[pousr]_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|sk-[A-Za-z0-9_-]{20,}|AKIA[0-9A-Z]{16})'; then
    hardening_fail "credential-shaped value found in fork-added lines"
fi

printf 'HARDENING CHECK OK\n'
printf '  upstream: %s (%s)\n' "$UPSTREAM_COMMIT" "$UPSTREAM_VERSION"
printf '  release:  v%s-hardened.%s\n' "$UPSTREAM_VERSION" "$HARDENED_REVISION"
