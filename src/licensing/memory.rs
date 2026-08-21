use std::ffi;

use crate::{
    hasp_api_ffi::{self, hasp_error_codes::HASP_STATUS_OK, hasp_size_t},
    licensing::{HaspError, HaspHandle},
};

pub const HASP_FILEID_RO: u32 = hasp_api_ffi::HASP_FILEID_RO;
pub const HASP_FILEID_RW: u32 = hasp_api_ffi::HASP_FILEID_RW;
// Memory

/// Retrieves the byte size of a memory file from a Sentinel protection key.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_get_size.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_get_size`]
///
pub fn hasp_get_size(handle: HaspHandle, file_id: u32) -> Result<u32, HaspError> {
    let mut size: hasp_size_t = 0;
    unsafe {
        let status = hasp_api_ffi::hasp_get_size(handle, file_id, &mut size);
        match status {
            HASP_STATUS_OK => Ok(size as u32),
            _ => Err(HaspError::from(status)),
        }
    }
}

/// Retrieves content from a memory file in a Sentinel protection key.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_read.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_read`]
///
pub fn hasp_read(
    handle: HaspHandle,
    file_id: u32,
    offset: u32,
    length: u32,
) -> Result<Vec<u8>, HaspError> {
    unsafe {
        let mut buffer = vec![0u8; length as usize];
        let status = hasp_api_ffi::hasp_read(
            handle,
            file_id,
            offset,
            buffer.len() as u32,
            buffer.as_mut_ptr() as *mut ffi::c_void,
        );
        match status {
            HASP_STATUS_OK => Ok(buffer),
            _ => Err(HaspError::from(status)),
        }
    }
}

/// Writes to the memory of a Sentinel protection key.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_write.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_write`]
///
pub fn hasp_write(
    handle: HaspHandle,
    file_id: u32,
    offset: u32,
    data: &Vec<u8>,
) -> Result<(), HaspError> {
    unsafe {
        let status = hasp_api_ffi::hasp_write(
            handle,
            file_id,
            offset,
            data.len() as u32,
            data.as_ptr() as *const ffi::c_void,
        );
        match status {
            HASP_STATUS_OK => Ok(()),
            _ => Err(HaspError::from(status)),
        }
    }
}
