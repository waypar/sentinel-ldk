# Rust Sentinel LDK Bindings

Unofficial Rust bindings for the Sentinel LDK / HASP Licensing API.

This project is not affiliated with or endorsed by Thales.

At build time, [bindgen](https://github.com/rust-lang/rust-bindgen) reads `hasp_api.h` from an installed Sentinel LDK SDK and links the matching static HASP library. The crate exposes the generated C API as [`hasp_api_ffi`](src/hasp_api_ffi.rs).

No Sentinel LDK files are included with this package. `SENTINEL_LDK_SDK_DIR` must be set at compile time. On Windows, if it is unset, the build script falls back to `Program Files (x86)\Thales\Sentinel LDK`.

## Usage

Add the crate from GitHub in your `Cargo.toml`:

```toml
[dependencies]
sentinel-ldk = { git = "https://github.com/waypar/sentinel-ldk.git" }
```

Use the bindings in your code:

```rust
use sentinel_ldk::hasp_api_ffi;

// All C API functions are unsafe — see Thales hasp_api.h / cargo doc
```

Point the build at your SDK and build your project (the dependency’s `build.rs` runs with these variables set):

```bash
export SENTINEL_LDK_SDK_DIR="/path/to/Sentinel-LDK"
export SENTINEL_LDK_VENDOR_ID="demo"   # optional; default is demo
cargo build
```

To try this repository directly:

```bash
git clone https://github.com/waypar/sentinel-ldk.git
cd sentinel-ldk
export SENTINEL_LDK_SDK_DIR="/path/to/Sentinel-LDK"
cargo build
cargo run --example ffi_get_version -- "$SENTINEL_LDK_SDK_DIR/VendorCodes/DEMOMA.hvc"
```

## Prerequisites

- Sentinel LDK SDK (see below)
- Clang / libclang (for bindgen)

### Obtaining the SDK

- **With a developer or master key:** install via the Sentinel Master Wizard.
- **Without a key (evaluation):** download the Sentinel LDK SDK from the [Thales knowledge base article](https://supportportal.thalesgroup.com/csm?id=kb_article_view&sys_kb_id=c2241c1d1bb41890f12064606e4bcb3e&sysparm_article=KB0021845).

Additional resources:

- [Sentinel LDK documentation](https://docs.sentinel.thalesgroup.com/ldk/)
- [Thales Customer Support Portal](https://supportportal.thalesgroup.com/csm?id=csm_product)

After downloading, unzip or run the installer, then set `SENTINEL_LDK_SDK_DIR` to the **root of the extracted SDK** — the directory that contains `API/` (typically `Sentinel-LDK`).

**Tested with:** Sentinel LDK SDK **10.2** on macOS (aarch64). Other versions and platforms may work but are not guaranteed.

## Environment variables

| Variable                 | Required | Default | Purpose                                                             |
| ------------------------ | -------- | ------- | ------------------------------------------------------------------- |
| `SENTINEL_LDK_SDK_DIR`   | **Yes**  | —       | Path to the Sentinel-LDK SDK root                                   |
| `SENTINEL_LDK_VENDOR_ID` | No       | `demo`  | Static library suffix: `hasp_darwin_{id}`, `hasp_linux_{arch}_{id}` |

\*On Windows, defaults to the Thales install under Program Files (x86) when unset.

### VS Code rust-analyzer

```json
{
  "rust-analyzer.cargo.extraEnv": {
    "SENTINEL_LDK_SDK_DIR": "/path/to/Sentinel-LDK"
  }
}
```

## Supported platforms

| OS      | Arch                                   | Status                                                             |
|---------|----------------------------------------|--------------------------------------------------------------------|
| macOS   | aarch64, x86_64                        | Supported, only aarch64 is tested                                  |
| Linux   | x86_64, arm64, armel, armhf, armuclibc | Supported, only x86_64 is tested                                   |
| Windows | x86, x86_64                            | Supported, only x86_64 is tested                                   |

## Documentation

- **C API reference:** [Sentinel Licensing API (C)](https://docs.sentinel.thalesgroup.com/softwareandservices/ldk/LDKdocs/API-licensing/Licensing_API/Licensing_API_ref-c.htm)
- **Rust bindings:** `cargo doc --open` after a successful build (includes rustified `hasp_error_codes`)
- **In the SDK:** `Docs/` and HTML guides under `GSG_*_HTML/`

All `hasp_api_ffi` functions are `unsafe` and mirror C semantics. Callers must manage `CString` lifetimes, vendor code pointers, and status codes.

## Examples

[`ffi_get_version`](examples/ffi_get_version.rs) calls `hasp_get_version()` using a `.hvc` file path from the command line (see **Usage** above for the `cargo run` command).

## Runtime requirements

Many APIs need the **Sentinel License Manager (RTE)** installed and running locally, or reachable via scope XML (for example in `hasp_login_scope()`). Without RTE, calls often return `HASP_LOCAL_COMM_ERR`.

## Need help?

- [GitHub Issues](https://github.com/waypar/sentinel-ldk/issues) for this unofficial crate
- [Thales support](https://supportportal.thalesgroup.com/) for SDK and licensing product questions

## License

This crate’s Rust sources are licensed under the [MIT License](LICENSE).

The Sentinel LDK SDK and HASP libraries are proprietary Thales software. Using this crate still requires a valid SDK install and compliance with Thales’ license terms.
