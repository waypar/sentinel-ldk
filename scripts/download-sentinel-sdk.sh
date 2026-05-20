#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "Usage: $0 <linux|windows> [--probe-only]" >&2
  exit 1
}

if [ $# -lt 1 ]; then
  usage
fi

platform="$1"
probe_only=false
if [ "${2:-}" = "--probe-only" ]; then
  probe_only=true
elif [ -n "${2:-}" ]; then
  usage
fi

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=sentinel-sdk-urls.env
source "${script_dir}/sentinel-sdk-urls.env"

url_var="${platform}_url"
sdk_url="${!url_var:-}"

if [ -z "${sdk_url}" ]; then
  echo "Missing URL for ${platform} in sentinel-sdk-urls.env" >&2
  exit 1
fi

url_path="${sdk_url%%\?*}"
archive_basename="${url_path##*/}"
if [ -z "${archive_basename}" ]; then
  echo "Could not derive archive basename from URL: ${sdk_url}" >&2
  exit 1
fi

repo_root="$(cd "${script_dir}/.." && pwd)"
cache_dir="${repo_root}/vendor/cache/${platform}"
mkdir -p "${cache_dir}"

remote_lm="$(curl -fsSLI "${sdk_url}" | tr -d '\r' | awk -F': ' '
  tolower($1) == "last-modified" {
    $1 = ""
    sub(/^: */, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    print
    exit
  }
')"

if [ -z "${remote_lm}" ]; then
  echo "Last-Modified header missing from ${sdk_url}" >&2
  exit 1
fi

parse_last_modified() {
  local lm="$1"
  if cache_tag="$(date -u -d "${lm}" +%Y%m%dT%H%M%SZ 2>/dev/null)" && [ -n "${cache_tag}" ]; then
    echo "${cache_tag}"
    return 0
  fi
  if [ "$(uname -s)" = "Darwin" ]; then
    date -u -jf "Fri, %d %b %Y %H:%M:%S GMT" "${lm}" +%Y%m%dT%H%M%SZ 2>/dev/null
    return $?
  fi
  return 1
}

cache_tag="$(parse_last_modified "${remote_lm}" || true)"
if [ -z "${cache_tag}" ]; then
  echo "Failed to parse Last-Modified: ${remote_lm}" >&2
  exit 1
fi

case "${archive_basename}" in
  *.tar.gz)
    archive_stem="${archive_basename%.tar.gz}"
    archive_ext="tar.gz"
    ;;
  *.zip)
    archive_stem="${archive_basename%.zip}"
    archive_ext="zip"
    ;;
  *)
    echo "Unsupported archive basename: ${archive_basename}" >&2
    exit 1
    ;;
esac
versioned_name="${archive_stem}-${cache_tag}.${archive_ext}"
archive_path="${cache_dir}/${versioned_name}"
manifest_path="${cache_dir}/latest.env"

write_manifest() {
  cat > "${manifest_path}" <<EOF
CACHE_TAG=${cache_tag}
LAST_MODIFIED="${remote_lm}"
ARCHIVE_PATH=${archive_path#${repo_root}/}
EOF
}

write_github_output() {
  if [ -n "${GITHUB_OUTPUT:-}" ]; then
    {
      echo "cache_tag=${cache_tag}"
      echo "platform=${platform}"
    } >> "${GITHUB_OUTPUT}"
  fi
}

echo "Platform: ${platform}"
echo "SDK URL: ${sdk_url}"
echo "Last-Modified: ${remote_lm}"
echo "Cache tag: ${cache_tag}"

write_github_output
write_manifest

if [ "${probe_only}" = true ]; then
  echo "Probe only; skipping download"
  exit 0
fi

if [ -f "${archive_path}" ]; then
  echo "Archive already cached: ${archive_path}"
  exit 0
fi

echo "Downloading SDK to ${archive_path}..."
curl -fL "${sdk_url}" -o "${archive_path}"

# Remove older versioned archives for this platform
shopt -s nullglob
for old_archive in "${cache_dir}/${archive_stem}-"*.${archive_ext}; do
  if [ "${old_archive}" != "${archive_path}" ]; then
    echo "Removing outdated archive: ${old_archive}"
    rm -f "${old_archive}"
  fi
done
shopt -u nullglob

write_manifest
echo "Download complete: ${archive_path}"
