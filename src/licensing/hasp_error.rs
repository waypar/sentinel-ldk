use crate::hasp_api_ffi;
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/hasp_error_messages.rs"));

pub type HaspStatus = hasp_api_ffi::hasp_error_codes;

/// Sentinel API error, which includes the status code, symbolic name, and human-readable message.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HaspError(HaspStatus);

impl HaspError {
    pub fn code(self) -> HaspStatus {
        self.0
    }

    /// Symbolic status name from Sentinel `hasp_api.h` (for example `HASP_INV_VCODE`).
    pub fn name(self) -> &'static str {
        hasp_status_name(self.0 as u32)
    }

    /// Human-readable description from Sentinel `hasp_api.h` status code comments.
    pub fn message(self) -> &'static str {
        hasp_status_message(self.0 as u32)
    }
}

impl From<HaspStatus> for HaspError {
    fn from(value: HaspStatus) -> Self {
        Self(value)
    }
}

impl PartialEq<HaspStatus> for HaspError {
    fn eq(&self, other: &HaspStatus) -> bool {
        self.0 == *other
    }
}

impl From<HaspError> for HaspStatus {
    fn from(value: HaspError) -> Self {
        value.0
    }
}

impl fmt::Display for HaspError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}): {}", self.name(), self.0 as u32, self.message())
    }
}

impl std::error::Error for HaspError {}

#[test]
fn test_hasp_error() {
    let error = HaspError::from(HaspStatus::HASP_INV_VCODE);
    assert_eq!(error.code(), HaspStatus::HASP_INV_VCODE);
    assert_eq!(error.name(), "HASP_INV_VCODE");
    assert_eq!(
        error.message(),
        "An invalid vendor code was passed to a function"
    );

    // PartialEq<HaspStatus> for HaspError
    assert!(error == HaspStatus::HASP_INV_VCODE);

    // From<HaspError> for HaspStatus
    assert_eq!(HaspError::from(HaspStatus::HASP_INV_VCODE), error);

    // From<HaspStatus> for HaspError
    assert_eq!(HaspStatus::from(error), HaspStatus::HASP_INV_VCODE);

    // fmt::Display for HaspError
    let error_string = format!("{}", error);
    println!("error_string: {}", error_string);
    assert_eq!(
        error_string,
        "HASP_INV_VCODE (22): An invalid vendor code was passed to a function"
    );
}
