use sentinel_ldk::hasp_api_ffi;
use std::ffi::CString;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <vendor_code_path>", args[0]);
        std::process::exit(1);
    }
    let vendor_code_path = &args[1];
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
    }
}
