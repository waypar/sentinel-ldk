#!/bin/bash
set -uo pipefail

if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

repo_root="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
cd "${repo_root}"

sdk_dir="${repo_root}/vendor/linux/Sentinel-LDK"

if [ -d "${sdk_dir}/API" ] && [ -d "${sdk_dir}/VendorCodes" ]; then
  echo "Sentinel LDK SDK already present at ${sdk_dir}"
else
  echo "Fetching Sentinel LDK SDK for Linux..."
  if ./scripts/download-sentinel-sdk.sh linux && ./scripts/install-sentinel-linux.sh; then
    echo "Sentinel LDK SDK installed at ${sdk_dir}"
  else
    echo "WARNING: Could not download/install the Sentinel LDK SDK (likely blocked by network policy for dlm.thalesgroup.com)."
    echo "WARNING: 'cargo build'/'cargo test' will fail with 'SENTINEL_LDK_SDK_DIR environment variable is not set' until this host is reachable or SENTINEL_LDK_SDK_DIR is set manually."
  fi
fi

if [ -d "${sdk_dir}/API" ]; then
  echo "export SENTINEL_LDK_SDK_DIR=\"${sdk_dir}\"" >> "${CLAUDE_ENV_FILE}"
fi

exit 0
