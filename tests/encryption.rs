use std::{env, fs, path::PathBuf};

use sentinel_ldk::licensing::{self, HaspEncryptError};

#[test]
fn hasp_encrypt() {
    let ldk_sdk_dir =
        PathBuf::from(env::var_os("SENTINEL_LDK_SDK_DIR").expect("SENTINEL_LDK_SDK_DIR unset"));

    let vendor_code_path = &ldk_sdk_dir.join("VendorCodes").join("DEMOMA.hvc");
    println!("Vendor code path: {}", vendor_code_path.display());
    let vendor_code = fs::read_to_string(vendor_code_path).expect("Reading vendor code failed");

    let handle = licensing::hasp_login(0, vendor_code).unwrap();

    let mut buf = vec![0u8; 4294967296];
    let result = licensing::hasp_encrypt(handle, &mut buf);
    println!(
        "hasp_encrypt() with oversized buffer returned expected error: {:?}",
        result
    );
    match result.as_ref().unwrap_err() {
        HaspEncryptError::OversizedBuffer(err) => {
            println!("Oversized buffer error: {:?}", err);
            assert!(
                err.to_string().contains("out of range"),
                "Unexpected error message: {}",
                err
            );
        }
        _ => {
            panic!("Expected OversizedBuffer error, got: {:?}", result);
        }
    }
    // assert!(matches!(
    //     result.unwrap_err(),
    //     HaspEncryptError::OversizedBuffer(_)
    // ));
}

#[test]
fn test_encryption() {
    // Test encryption and decryption functions
    let ldk_sdk_dir =
        PathBuf::from(env::var_os("SENTINEL_LDK_SDK_DIR").expect("SENTINEL_LDK_SDK_DIR unset"));

    let vendor_code_path = &ldk_sdk_dir.join("VendorCodes").join("DEMOMA.hvc");
    println!("Vendor code path: {}", vendor_code_path.display());
    let vendor_code = fs::read_to_string(vendor_code_path).expect("Reading vendor code failed");

    let handle = licensing::hasp_login(0, &vendor_code).unwrap();
    let mut buffer = b"This must be at least 16 bytes - Hello, World!".to_vec();
    let original_buffer = buffer.clone();
    licensing::hasp_encrypt(handle, &mut buffer).unwrap();
    assert_ne!(buffer, original_buffer);
    println!("Encrypted buffer: {:?}", buffer);
    licensing::hasp_decrypt(handle, &mut buffer).unwrap();
    assert_eq!(buffer, original_buffer);
}
