use crate::hasp_api_ffi;
use crate::licensing::{HaspError, HaspHandle, HaspStatus};
use std::ffi::CString;

// Session

/// Logs into a Feature, which establishes a session context.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_login.htm>
///
/// [`crate::hasp_api_ffi::hasp_login`]
///
pub fn hasp_login(feature_id: u32, vendor_code: impl AsRef<str>) -> Result<HaspHandle, HaspError> {
    let vendor_code = CString::new(vendor_code.as_ref()).map_err(|_| HaspStatus::HASP_INV_VCODE)?;
    let mut handle: HaspHandle = 0;

    unsafe {
        let status = hasp_api_ffi::hasp_login(
            feature_id,
            vendor_code.as_ptr() as hasp_api_ffi::hasp_vendor_code_t,
            &mut handle,
        );
        match status {
            HaspStatus::HASP_STATUS_OK => Ok(handle),
            _ => Err(status.into()),
        }
    }
}

/// Logs into a Feature to establish a session, according to predefined search parameters.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_login_scope.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_login_scope`]
///
pub fn hasp_login_scope(
    feature_id: u32,
    scope: impl AsRef<str>,
    vendor_code: impl AsRef<str>,
) -> Result<HaspHandle, HaspError> {
    let scope = CString::new(scope.as_ref()).map_err(|_| HaspStatus::HASP_INV_SCOPE)?;
    let vendor_code = CString::new(vendor_code.as_ref()).map_err(|_| HaspStatus::HASP_INV_VCODE)?;
    let mut handle: HaspHandle = 0;

    unsafe {
        let status = hasp_api_ffi::hasp_login_scope(
            feature_id,
            scope.as_ptr(),
            vendor_code.as_ptr() as hasp_api_ffi::hasp_vendor_code_t,
            &mut handle,
        );
        match status {
            HaspStatus::HASP_STATUS_OK => Ok(handle),
            _ => Err(status.into()),
        }
    }
}

/// Logs out from a context or session.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_logout.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_logout`]
///
pub fn hasp_logout() -> Result<(), HaspError> {
    Err(HaspStatus::HASP_NOT_IMPL.into())
}

/// Retrieves information about a session.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_get_session_info.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_get_session_info`]
///
pub fn hasp_get_session_info() -> Result<(), HaspError> {
    Err(HaspStatus::HASP_NOT_IMPL.into())
}

/// Update information regarding a login session for a Sentinel protection key.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_update_session.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_update_session`]
///
pub fn hasp_update_session() -> Result<(), HaspError> {
    Err(HaspStatus::HASP_NOT_IMPL.into())
}
