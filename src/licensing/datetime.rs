use crate::{
    hasp_api_ffi::{self, hasp_time_t},
    licensing::{HaspDatetime, HaspError, HaspHandle, HaspStatus},
};

// datetime

/// Reads the current time from a Sentinel HL Time key, a Sentinel HL NetTime key, or a Sentinel HL (Driverless configuration) key that supports V-Clock.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_get_rtc.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_get_rtc`]
///
pub fn hasp_get_rtc(handle: HaspHandle) -> Result<u64, HaspError> {
    let mut time: hasp_time_t = 0;
    unsafe {
        let status = hasp_api_ffi::hasp_get_rtc(handle, &mut time);
        match status {
            HaspStatus::HASP_STATUS_OK => Ok(time),
            _ => Err(HaspError::from(status)),
        }
    }
}
/// Converts a date and time value to hasptime (the number of elapsed seconds since January 1 1970).
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_datetime_to_hasptime.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_datetime_to_hasptime`]
///
pub fn hasp_datetime_to_hasptime(
    day: u32,
    month: u32,
    year: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> Result<u64, HaspError> {
    let mut time: hasp_time_t = 0;
    unsafe {
        let status = hasp_api_ffi::hasp_datetime_to_hasptime(
            day, month, year, hour, minute, second, &mut time,
        );
        match status {
            HaspStatus::HASP_STATUS_OK => Ok(time),
            _ => Err(HaspError::from(status)),
        }
    }
}

/// Converts a time value (elapsed seconds since January 1, 1970) into a date and time.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_hasptime_to_datetime.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_hasptime_to_datetime`]
///
pub fn hasp_hasptime_to_datetime(time: u64) -> Result<HaspDatetime, HaspError> {
    unsafe {
        let mut date_time = HaspDatetime {
            day: 0,
            month: 0,
            year: 0,
            hour: 0,
            minute: 0,
            second: 0,
        };
        let result = hasp_api_ffi::hasp_hasptime_to_datetime(
            time,
            &mut date_time.day,
            &mut date_time.month,
            &mut date_time.year,
            &mut date_time.hour,
            &mut date_time.minute,
            &mut date_time.second,
        );
        match result {
            HaspStatus::HASP_STATUS_OK => Ok(date_time),
            _ => Err(HaspError::from(result)),
        }
    }
}
