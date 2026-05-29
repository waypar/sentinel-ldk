use crate::hasp_api_ffi;
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
    fmt,
};

pub type HaspHandle = hasp_api_ffi::hasp_handle_t;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HaspVersion {
    pub major_version: u32,
    pub minor_version: u32,
    pub generation_version: u32,
    pub build_number: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct HaspDatetime {
    pub day: u32,
    pub month: u32,
    pub year: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl fmt::Display for HaspVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major_version, self.minor_version, self.generation_version, self.build_number
        )
    }
}

#[derive(Debug)]
pub enum HaspScope {
    Custom(String),
}

impl AsRef<HaspScope> for HaspScope {
    fn as_ref(&self) -> &HaspScope {
        self
    }
}
// impl AsRef<&str> for HaspScope {
//     fn as_ref(s: &str) -> HaspScope {
//         HaspScope::Custom(s.to_string())
//     }
// }
impl From<&str> for HaspScope {
    fn from(s: &str) -> Self {
        HaspScope::Custom(s.to_string())
    }
}
impl From<String> for HaspScope {
    fn from(s: String) -> Self {
        HaspScope::Custom(s)
    }
}
impl HaspScope {
    pub fn as_cstr(&self) -> CString {
        match self {
            HaspScope::Custom(s) => CString::new(s.split('\0').next().unwrap_or("")).unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum HaspFormat {
    UpdateInfo,
    FastUpdateInfo,
    SessionInfo,
    KeyInfo,
    Fingerprint,
    Recipient,
    MachineInfo,
    Custom(String),
}

impl HaspFormat {
    pub fn as_cstr(&self) -> Cow<'_, CStr> {
        match self {
            HaspFormat::UpdateInfo => unsafe {
                Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(
                    hasp_api_ffi::HASP_UPDATEINFO,
                ))
            },
            HaspFormat::FastUpdateInfo => unsafe {
                Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(
                    hasp_api_ffi::HASP_FASTUPDATEINFO,
                ))
            },
            HaspFormat::SessionInfo => unsafe {
                Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(
                    hasp_api_ffi::HASP_SESSIONINFO,
                ))
            },
            HaspFormat::KeyInfo => unsafe {
                Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(
                    hasp_api_ffi::HASP_KEYINFO,
                ))
            },
            HaspFormat::Fingerprint => unsafe {
                Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(
                    hasp_api_ffi::HASP_FINGERPRINT,
                ))
            },
            HaspFormat::Recipient => unsafe {
                Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(
                    hasp_api_ffi::HASP_RECIPIENT,
                ))
            },
            HaspFormat::MachineInfo => unsafe {
                Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(
                    hasp_api_ffi::HASP_MACHINEINFO,
                ))
            },
            HaspFormat::Custom(s) => {
                Cow::Owned(CString::new(s.split('\0').next().unwrap_or("")).unwrap())
            }
        }
    }
}
