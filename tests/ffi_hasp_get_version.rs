use sentinel_ldk::hasp_api_ffi;
use std::ffi::CString;
use std::path::PathBuf;
use std::{env, fs};

#[test]
fn ffi_hasp_get_version() {
    let ldk_sdk_dir =
        PathBuf::from(env::var_os("SENTINEL_LDK_SDK_DIR").expect("SENTINEL_LDK_SDK_DIR unset"));

    let vendor_code_path = &ldk_sdk_dir.join("VendorCodes").join("DEMOMA.hvc");
    println!("Vendor code path: {}", vendor_code_path.display());
    let vendor_code =
        CString::new(fs::read_to_string(vendor_code_path).expect("Reading vendor code failed"))
            .unwrap();

    let mut major_version = 0;
    let mut minor_version = 0;
    let mut generation_version = 0;
    let mut build_number = 0;

    println!("Calling hasp_get_version()");

    unsafe {
        let status = hasp_api_ffi::hasp_get_version(
            &mut major_version,
            &mut minor_version,
            &mut generation_version,
            &mut build_number,
            vendor_code.as_ref().as_ptr() as hasp_api_ffi::hasp_vendor_code_t,
        );
        println!("HASP Status: {:?}", status);
        if status == hasp_api_ffi::hasp_error_codes::HASP_STATUS_OK {
            println!(
                "HASP Version: {:?}.{:?}.{:?}.{:?}",
                major_version, minor_version, generation_version, build_number
            );
        }
        assert_eq!(major_version, 10);
        assert_eq!(generation_version, 8);
    }
}
