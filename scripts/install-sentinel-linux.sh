#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest_path="${repo_root}/vendor/cache/linux/latest.env"
SENTINEL_LDK_SDK_DIR="${repo_root}/vendor/linux/Sentinel-LDK"

if [ ! -f "${manifest_path}" ]; then
  echo "Missing ${manifest_path}; run ./scripts/download-sentinel-sdk.sh linux first" >&2
  exit 1
fi

# shellcheck source=/dev/null
source "${manifest_path}"

if [ -z "${ARCHIVE_PATH:-}" ]; then
  echo "ARCHIVE_PATH not set in ${manifest_path}" >&2
  exit 1
fi

archive_path="${repo_root}/${ARCHIVE_PATH}"
if [ ! -f "${archive_path}" ]; then
  echo "Archive not found: ${archive_path}; run ./scripts/download-sentinel-sdk.sh linux first" >&2
  exit 1
fi

mkdir -p "${repo_root}/vendor/linux"

if [ ! -d "${SENTINEL_LDK_SDK_DIR}/API" ] || [ ! -d "${SENTINEL_LDK_SDK_DIR}/VendorCodes" ]; then
  rm -rf "${SENTINEL_LDK_SDK_DIR}"
  tar -xzf "${archive_path}" -C "${repo_root}/vendor/linux" "Sentinel-LDK/API" "Sentinel-LDK/VendorCodes"
fi

if [ -n "${GITHUB_ENV:-}" ]; then
  echo "SENTINEL_LDK_SDK_DIR=${SENTINEL_LDK_SDK_DIR}" >> "${GITHUB_ENV}"
fi
