use sentinel_ldk::licensing;
use std::env;

#[test]
fn test_memory() {
    // Test memory functions
    let ldk_sdk_dir = std::path::PathBuf::from(
        env::var_os("SENTINEL_LDK_SDK_DIR").expect("SENTINEL_LDK_SDK_DIR unset"),
    );
    let vendor_code_path = &ldk_sdk_dir.join("VendorCodes").join("DEMOMA.hvc");
    println!("Vendor code path: {}", vendor_code_path.display());
    let vendor_code =
        std::fs::read_to_string(vendor_code_path).expect("Reading vendor code failed");

    let handle = licensing::hasp_login(0, &vendor_code).unwrap();

    let initial_size = licensing::hasp_get_size(handle, licensing::HASP_FILEID_RW).unwrap();
    println!("Initial size of RW memory file: {}", initial_size);

    {
        // Reset the contents
        let empty_vec = vec![0; initial_size.try_into().unwrap()];
        let write_result = licensing::hasp_write(handle, licensing::HASP_FILEID_RW, 0, &empty_vec);
        assert!(write_result.is_ok());
    }
    let data_to_write = "Hello World!".as_bytes().to_vec();

    let write_result = licensing::hasp_write(handle, licensing::HASP_FILEID_RW, 0, &data_to_write);
    assert!(write_result.is_ok());

    let read_result = licensing::hasp_read(
        handle,
        licensing::HASP_FILEID_RW,
        0,
        data_to_write.len().try_into().unwrap(),
    )
    .unwrap();
    assert_eq!(&read_result, &data_to_write);
}
