use sentinel_ldk::licensing;
use std::path::PathBuf;
use std::{env, fs};

const EXAMPLE_TIMESTAMP: u64 = 1781940300;
const EXAMPLE_DATETIME: licensing::HaspDatetime = licensing::HaspDatetime {
    year: 2026,
    month: 6,
    day: 20,
    hour: 7,
    minute: 25,
    second: 0,
};

#[test]
fn hasp_get_rtc() {
    let ldk_sdk_dir =
        PathBuf::from(env::var_os("SENTINEL_LDK_SDK_DIR").expect("SENTINEL_LDK_SDK_DIR unset"));

    let vendor_code_path = &ldk_sdk_dir.join("VendorCodes").join("DEMOMA.hvc");
    println!("Vendor code path: {}", vendor_code_path.display());
    let vendor_code = fs::read_to_string(vendor_code_path).expect("Reading vendor code failed");

    println!("Calling hasp_get_rtc()");

    let session = licensing::hasp_login(0, vendor_code).expect("Login failed");
    let result = licensing::hasp_get_rtc(session);
    println!("result: {:?}", result);
    assert!(result.is_ok(), "hasp_get_rtc failed: {:?}", result);
}

#[test]
fn hasp_datetime_to_hasptime() {
    let result = licensing::hasp_datetime_to_hasptime(
        EXAMPLE_DATETIME.day,
        EXAMPLE_DATETIME.month,
        EXAMPLE_DATETIME.year,
        EXAMPLE_DATETIME.hour,
        EXAMPLE_DATETIME.minute,
        EXAMPLE_DATETIME.second,
    );
    println!("result: {:?}", result);
    assert_eq!(result, Ok(EXAMPLE_TIMESTAMP));
}

#[test]
fn hasp_hasptime_to_datetime() {
    let result = licensing::hasp_hasptime_to_datetime(EXAMPLE_TIMESTAMP);
    println!("result: {:?}", result);
    assert_eq!(result, Ok(EXAMPLE_DATETIME));
}
