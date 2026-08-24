#!/usr/bin/env bash
# Added by the grok-build-hardened project.
# Compatibility wrapper; the cross-platform implementation lives in hardening.py.

set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
python_command="$(command -v python3 || command -v python)"
exec "$python_command" "${script_dir}/hardening.py" build-offline "$@"
