use crate::licensing::{HaspError, HaspStatus};

// Memory

/// Retrieves the byte size of a memory file from a Sentinel protection key.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_get_size.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_get_size`]
///
pub fn hasp_get_size() -> Result<(), HaspError> {
    Err(HaspStatus::HASP_NOT_IMPL.into())
}

/// Retrieves content from a memory file in a Sentinel protection key.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_read.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_read`]
///
pub fn hasp_read() -> Result<(), HaspError> {
    Err(HaspStatus::HASP_NOT_IMPL.into())
}

/// Writes to the memory of a Sentinel protection key.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_write.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_write`]
///
pub fn hasp_write() -> Result<(), HaspError> {
    Err(HaspStatus::HASP_NOT_IMPL.into())
}
