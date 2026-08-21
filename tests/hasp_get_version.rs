use sentinel_ldk::licensing;
use std::path::PathBuf;
use std::{env, fs};

#[test]
fn hasp_get_version() {
    let ldk_sdk_dir =
        PathBuf::from(env::var_os("SENTINEL_LDK_SDK_DIR").expect("SENTINEL_LDK_SDK_DIR unset"));

    let vendor_code_path = &ldk_sdk_dir.join("VendorCodes").join("DEMOMA.hvc");
    println!("Vendor code path: {}", vendor_code_path.display());
    let vendor_code = fs::read_to_string(vendor_code_path).expect("Reading vendor code failed");

    println!("Calling hasp_get_version()");

    let version = licensing::hasp_get_version(&vendor_code).unwrap();
    println!("HASP Version: {}", version);

    assert_eq!(version.major_version, 10);
    assert_eq!(version.generation_version, 8);
}

#[test]
fn hasp_get_version_fails_with_null_char_vendor_code() {
    println!("Calling hasp_get_version()");

    let err = licensing::hasp_get_version("\0").unwrap_err();
    println!("HASP Version: {}", err);

    assert_eq!(err, licensing::HaspStatus::HASP_INV_VCODE);
    assert_eq!(err.name(), "HASP_INV_VCODE");
    assert!(err.message().contains("invalid vendor code"));
}
