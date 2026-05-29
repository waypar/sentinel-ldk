use std::ffi::CStr;

use crate::{
    hasp_api_ffi,
    licensing::{HaspError, HaspStatus},
};

/// Converts a hasp function output string to a Result<Option<String>, HaspError>.
pub fn hasp_out_string_to_result(
    result: HaspStatus,
    info: *mut std::os::raw::c_char,
) -> Result<Option<String>, HaspError> {
    match result {
        HaspStatus::HASP_STATUS_OK => {
            if info.is_null() {
                return Ok(None);
            }
            unsafe {
                let info_string = CStr::from_ptr(info).to_string_lossy().to_string();
                hasp_api_ffi::hasp_free(info);
                Ok(Some(info_string))
            }
        }
        _ => Err(result.into()),
    }
}
