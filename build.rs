mod build_hasp_error;
use build_hasp_error::{extract_hasp_error_details, generate_hasp_error_messages_rs};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-env-changed=SENTINEL_LDK_SDK_DIR");
    println!("cargo::rerun-if-env-changed=SENTINEL_LDK_VENDOR_ID");

    println!("The OS is: {}", std::env::consts::OS);
    println!("The architecture is: {}", std::env::consts::ARCH);
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_abi = std::env::var("CARGO_CFG_TARGET_ABI").unwrap_or_default();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    println!("The target OS is: {}", &target_os);
    println!("The target architecture is: {}", &target_arch);

    let sentinel_arch = match target_os.as_str() {
        "macos" => "",
        "linux" => match target_arch.as_str() {
            // Only x86_64 is tested
            "aarch64" => "arm64",
            // Thales SDK dirs: armel (soft float), armhf (eabihf), armuclibc (uclibc toolchain)
            "arm" => match target_env.as_str() {
                "uclibc" => "armuclibc",
                _ => match target_abi.as_str() {
                    "eabihf" => "armhf",
                    "" | "eabi" => "armel",
                    _ => panic!(
                        "Unsupported Linux ARM target (arch={target_arch}, abi={target_abi:?}, env={target_env:?})"
                    ),
                },
            },
            _ => target_arch.as_str(),
        },
        "windows" => match target_arch.as_str() {
            "x86_64" => "x64",
            "x86" => "win32",
            _ => target_arch.as_str(),
        },
        _ => panic!("Unsupported target"),
    };

    let ldk_sdk_dir = match env::var_os("SENTINEL_LDK_SDK_DIR") {
        Some(val) => PathBuf::from(val),
        None => {
            if std::env::consts::OS == "windows" {
                std::env::var_os("ProgramFiles(x86)")
                    .map(|v| PathBuf::from(v).join("Thales").join("Sentinel LDK"))
            } else {
                None
            }
        }
        .expect("SENTINEL_LDK_SDK_DIR environment variable is not set"),
    };

    let vendor_id = env::var("SENTINEL_LDK_VENDOR_ID").unwrap_or("demo".to_string());

    println!("Using LDK SDK from {:?}", ldk_sdk_dir);
    let ldk_api_dir = PathBuf::from(&ldk_sdk_dir).join("API");

    let hasp_api_h_path = match target_os.as_str() {
        "windows" => ldk_api_dir
            .join("Licensing")
            .join("C")
            .join(sentinel_arch)
            .join("hasp_api.h"),
        _ => ldk_api_dir.join("Licensing").join("C").join("hasp_api.h"),
    };

    // Generate the bindings for the Licensing API
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", ldk_api_dir.display()))
        .header(hasp_api_h_path.to_string_lossy().to_string())
        .rustified_enum("hasp_error_codes")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");

    // Generate hasp error companion functions
    let bindings_src = fs::read_to_string(&bindings_path)?;
    let details = extract_hasp_error_details(&bindings_src)?;
    generate_hasp_error_messages_rs(&details, &out_path.join("hasp_error_messages.rs"))?;

    let hasp_lib_name = match target_os.as_str() {
        "macos" => format!("hasp_darwin_{}", &vendor_id),
        "linux" | "windows" => format!("hasp_{}_{}_{}", &target_os, &sentinel_arch, &vendor_id),
        _ => panic!("Unsupported target"),
    };

    println!("cargo::rustc-link-lib=static={}", &hasp_lib_name);

    let lib_path = PathBuf::from(&ldk_api_dir)
        .join("Licensing/C")
        .join(sentinel_arch);
    println!("cargo::rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=static={}", &hasp_lib_name);

    if target_os == "macos" {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=SystemConfiguration");

        // if target_os == "macos" {
        //     println!("cargo:rustc-link-lib=framework=CoreFoundation");
        //     println!("cargo:rustc-link-lib=framework=Security");
        //     println!("cargo:rustc-link-lib=framework=SystemConfiguration");
        //     println!("cargo:rustc-link-lib=framework=IOKit");
        // }
    }

    Ok(())
}
