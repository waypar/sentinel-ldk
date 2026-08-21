use std::{ffi::c_void, num::TryFromIntError};

use crate::{
    hasp_api_ffi,
    licensing::{HaspError, HaspHandle, HaspStatus},
};

// Encryption

#[derive(Debug)]
pub enum HaspEncryptError {
    HaspError(HaspError),
    OversizedBuffer(TryFromIntError),
}

/// Encrypts a buffer.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_encrypt.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_encrypt`]
///
pub fn hasp_encrypt(handle: HaspHandle, buffer: &mut [u8]) -> Result<(), HaspEncryptError> {
    unsafe {
        let buffer_length: u32 = buffer
            .len()
            .try_into()
            .map_err(|err| HaspEncryptError::OversizedBuffer(err))?;
        let status =
            hasp_api_ffi::hasp_encrypt(handle, buffer.as_mut_ptr() as *mut c_void, buffer_length);
        match status {
            HaspStatus::HASP_STATUS_OK => Ok(()),
            _ => Err(HaspEncryptError::HaspError(HaspError::from(status))),
        }
    }
}

/// Reverses the operation of the Encrypt function applied on a data buffer, returning the data to its unencrypted state.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_decrypt.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_decrypt`]
///
pub fn hasp_decrypt(handle: HaspHandle, buffer: &mut [u8]) -> Result<(), HaspEncryptError> {
    unsafe {
        let buffer_length: u32 = buffer
            .len()
            .try_into()
            .map_err(|err| HaspEncryptError::OversizedBuffer(err))?;
        let status =
            hasp_api_ffi::hasp_decrypt(handle, buffer.as_mut_ptr() as *mut c_void, buffer_length);
        match status {
            HaspStatus::HASP_STATUS_OK => Ok(()),
            _ => Err(HaspEncryptError::HaspError(HaspError::from(status))),
        }
    }
}
