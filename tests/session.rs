use sentinel_ldk::licensing;
use std::path::PathBuf;
use std::{env, fs};

#[test]
fn hasp_login() {
    let ldk_sdk_dir =
        PathBuf::from(env::var_os("SENTINEL_LDK_SDK_DIR").expect("SENTINEL_LDK_SDK_DIR unset"));

    let vendor_code_path = &ldk_sdk_dir.join("VendorCodes").join("DEMOMA.hvc");
    println!("Vendor code path: {}", vendor_code_path.display());
    let vendor_code = fs::read_to_string(vendor_code_path).expect("Reading vendor code failed");

    let license = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("licenses")
            .join("Unlocked_20260529_182601.v2c"),
    )
    .expect("Reading license file failed");

    println!("Calling hasp_update()");
    let license_result = licensing::hasp_update(license);

    println!("License update result: {:?}", license_result);

    println!("Calling hasp_login()");
    let handle = licensing::hasp_login(0, &vendor_code);

    assert!(handle.is_ok());
}

#[test]
fn hasp_login_fails_with_null_char_vendor_code() {
    println!("Calling hasp_login()");

    let err = licensing::hasp_login(0, "\0").unwrap_err();
    println!("HASP Version: {}", err);

    assert_eq!(err, licensing::HaspStatus::HASP_INV_VCODE);
    assert_eq!(err.name(), "HASP_INV_VCODE");
    assert!(err.message().contains("invalid vendor code"));
}
