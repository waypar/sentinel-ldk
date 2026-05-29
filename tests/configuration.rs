use sentinel_ldk::licensing::{self, HaspFormat, HaspScope, HaspTransferAction};
use std::path::PathBuf;
use std::{env, fs};

#[test]
fn hasp_get_info() {
    let ldk_sdk_dir =
        PathBuf::from(env::var_os("SENTINEL_LDK_SDK_DIR").expect("SENTINEL_LDK_SDK_DIR unset"));

    let vendor_code_path = &ldk_sdk_dir.join("VendorCodes").join("DEMOMA.hvc");
    println!("Vendor code path: {}", vendor_code_path.display());
    let vendor_code = fs::read_to_string(vendor_code_path).expect("Reading vendor code failed");

    let scope = HaspScope::from("<haspscope />");
    for format in [
        // HaspFormat::UpdateInfo,
        // HaspFormat::FastUpdateInfo,
        HaspFormat::Fingerprint,
        HaspFormat::Recipient,
        HaspFormat::MachineInfo,
    ] {
        println!(
            "Calling hasp_get_info() with format: {:?}, scope: {:?}",
            format, &scope
        );
        let info =
            licensing::hasp_get_info(&scope, &format, &vendor_code).expect("hasp_get_info failed");

        println!(
            "Info for format {:?}: {}",
            format,
            info.as_deref().unwrap_or("None")
        );
        assert!((&info.unwrap()).len() > 0);
    }
}

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

#[test]
fn hasp_transfer() {
    let ldk_sdk_dir =
        PathBuf::from(env::var_os("SENTINEL_LDK_SDK_DIR").expect("SENTINEL_LDK_SDK_DIR unset"));

    let vendor_code_path = &ldk_sdk_dir.join("VendorCodes").join("DEMOMA.hvc");
    println!("Vendor code path: {}", vendor_code_path.display());
    let vendor_code = fs::read_to_string(vendor_code_path).expect("Reading vendor code failed");

    println!("Calling hasp_transfer()");

    let result = licensing::hasp_transfer(
        HaspTransferAction::Custom("<revoke />"),
        "",
        &vendor_code,
        "",
    );
    println!("result: {:?}", result);
}

#[test]
fn hasp_transfer_revoke() {
    let ldk_sdk_dir =
        PathBuf::from(env::var_os("SENTINEL_LDK_SDK_DIR").expect("SENTINEL_LDK_SDK_DIR unset"));

    let vendor_code_path = &ldk_sdk_dir.join("VendorCodes").join("DEMOMA.hvc");
    println!("Vendor code path: {}", vendor_code_path.display());
    let vendor_code = fs::read_to_string(vendor_code_path).expect("Reading vendor code failed");

    println!("Calling hasp_transfer()");

    let result = licensing::hasp_transfer(
        HaspTransferAction::Rehost {
            key_id: "1041194312530049815",
        },
        "",
        &vendor_code,
        "",
    );
    println!("result: {:?}", result);
}

#[test]
fn hasp_config() {
    let ldk_sdk_dir =
        PathBuf::from(env::var_os("SENTINEL_LDK_SDK_DIR").expect("SENTINEL_LDK_SDK_DIR unset"));

    let vendor_code_path = &ldk_sdk_dir.join("VendorCodes").join("DEMOMA.hvc");
    println!("Vendor code path: {}", vendor_code_path.display());
    let vendor_code = fs::read_to_string(vendor_code_path).expect("Reading vendor code failed");

    println!("Calling hasp_config()");

    let result = licensing::hasp_config(
        licensing::HaspConfigConfig::MachineAccount {
            identity_string: Some(String::from("hi<")),
            clear: true,
            usage_data: None,
        },
        &vendor_code,
    );
    println!("result: {:?}", result);
    let result = licensing::hasp_config("some static str", &vendor_code);
    println!("result: {:?}", result);
    let result = licensing::hasp_config(String::from("some string"), &vendor_code);
    println!("result: {:?}", result);
}
