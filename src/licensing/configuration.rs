use super::hasp_error::{HaspError, HaspStatus};
use super::types::HaspVersion;
use super::util::hasp_out_string_to_result;
use crate::hasp_api_ffi::{self, hasp_vendor_code_t};
use crate::licensing::{HaspFormat, HaspScope};
use std::ffi::CString;
use std::ptr::null_mut;
use xml;
use xml::escape::escape_str_pcdata;

// Configuration

/// Retrieves information about system components, according to customizable search parameters, and presents it according to customizable formats.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_get_info.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_get_info`]
///
pub fn hasp_get_info(
    scope: &HaspScope,
    format: &HaspFormat,
    vendor_code: impl AsRef<str>,
) -> Result<Option<String>, HaspError> {
    unsafe {
        let vendor_code_cstr =
            CString::new(vendor_code.as_ref()).map_err(|_| HaspStatus::HASP_INV_VCODE)?;
        let mut info: *mut std::os::raw::c_char = null_mut();

        let status = hasp_api_ffi::hasp_get_info(
            scope.as_cstr().as_ptr(),
            format.as_cstr().as_ptr(),
            vendor_code_cstr.as_ptr() as hasp_api_ffi::hasp_vendor_code_t,
            &mut info,
        );

        hasp_out_string_to_result(status, info)
    }
}

/// Write an update to a local or remote Sentinel protection key.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_update.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_update`]
///
pub fn hasp_update(update_data: impl AsRef<str>) -> Result<Option<String>, HaspError> {
    let update_data = CString::new(update_data.as_ref()).map_err(|_| HaspStatus::HASP_INV_VCODE)?;

    let mut ack_data: *mut std::os::raw::c_char = null_mut();

    unsafe {
        let status = hasp_api_ffi::hasp_update(update_data.as_ptr(), &mut ack_data);
        hasp_out_string_to_result(status, ack_data)
    }
}

#[derive(Debug, PartialEq)]
pub enum HaspTransferAction<'a> {
    Rehost {
        key_id: &'a str,
    },
    Detach {
        product_id: &'a str,
        duration: u32,
    },
    NetworkDetach {
        product_id: &'a str,
        duration: u32,
        seats: u32,
    },
    CancelDetach {
        key_id: &'a str,
    },
    Revoke {
        product_id: &'a str,
    },
    Custom(&'a str),
}

/// Transfers a license from a Sentinel SL key to a recipient machine or revokes a license from a Sentinel SL key.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_transfer.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_transfer`]
///
pub fn hasp_transfer(
    action: HaspTransferAction,
    scope: impl AsRef<str>,
    vendor_code: impl AsRef<str>,
    recipient: impl AsRef<str>,
) -> Result<Option<String>, HaspError> {
    match action {
        HaspTransferAction::Rehost { ref key_id } => {
            println!("Performing Rehost transfer for key_id: {}", key_id);
        }
        HaspTransferAction::Detach {
            ref product_id,
            duration,
        } => {
            println!(
                "Performing Detach transfer for product_id: {}, duration: {} days",
                product_id, duration
            );
        }
        HaspTransferAction::NetworkDetach {
            ref product_id,
            duration,
            seats,
        } => {
            println!(
                "Performing Network Detach transfer for product_id: {}, duration: {} days, seats: {}",
                product_id, duration, seats
            );
        }
        HaspTransferAction::CancelDetach { ref key_id } => {
            println!("Performing Cancel Detach transfer for key_id: {}", key_id);
        }
        HaspTransferAction::Revoke { ref product_id } => {
            println!("Performing Revoke transfer for product_id: {}", product_id);
        }
        HaspTransferAction::Custom(ref action_xml) => {
            println!("Performing Custom transfer with action_xml: {}", action_xml);
        }
    }
    let action_xml = match action {
        HaspTransferAction::Rehost { ref key_id } => {
            CString::new(format!("<rehost> <hasp id=\"{key_id}\"/> </rehost>"))
        }
        HaspTransferAction::Detach {
            ref product_id,
            duration,
        } => CString::new(format!(
            "<detach> <product id=\"{product_id}\"> <duration>{duration}</duration> </product> </detach>"
        )),
        HaspTransferAction::NetworkDetach {
            ref product_id,
            duration,
            seats,
        } => CString::new(format!(
            "<network_detach> <product id=\"{product_id}\"> <duration>{duration}</duration> <seats>{seats}</seats> </product> </network_detach>"
        )),
        HaspTransferAction::CancelDetach { ref key_id } => {
            CString::new(format!("<cancel> <hasp id=\"{key_id}\"/> </cancel>"))
        }

        HaspTransferAction::Revoke { ref product_id } => {
            CString::new(format!("<revoke><product id=\"{product_id}\"/></revoke>"))
        }
        HaspTransferAction::Custom(action_xml) => CString::new(action_xml),
    };
    let action_xml_cstr = action_xml.map_err(|_| HaspStatus::HASP_INV_ACTION)?;
    println!("Constructed action XML: {:?}", action_xml_cstr);
    let scope_cstr = CString::new(scope.as_ref()).map_err(|_| HaspStatus::HASP_INV_SCOPE)?;
    let vendor_code_cstr =
        CString::new(vendor_code.as_ref()).map_err(|_| HaspStatus::HASP_INV_VCODE)?;
    let recipient_cstr =
        CString::new(recipient.as_ref()).map_err(|_| HaspStatus::HASP_INV_RECIPIENT)?;
    unsafe {
        let mut info: *mut std::os::raw::c_char = null_mut();
        let status = hasp_api_ffi::hasp_transfer(
            action_xml_cstr.as_ptr(),
            scope_cstr.as_ptr(),
            vendor_code_cstr.as_ptr() as hasp_api_ffi::hasp_vendor_code_t,
            recipient_cstr.as_ptr(),
            &mut info,
        );
        hasp_out_string_to_result(status, info)
    }
}

pub enum UserOption {
    OptIn,
    OptOut,
}

pub struct UsageData {
    user_option: Option<UserOption>,
    app_name: Option<String>,
    app_version: Option<String>,
}
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_XML_Tags/HASP_CONFIG_XML_Tags.htm>
pub enum HaspConfigConfig {
    MachineAccount {
        identity_string: Option<String>,
        clear: bool,
        usage_data: Option<UsageData>,
    },
    ServerCertificate {
        certificate: String,
        certificate_store: Option<String>,
        usage_data: Option<UsageData>,
    },
    AuthRT {
        enabled: bool,
        usage_data: Option<UsageData>,
    },
    JWT {
        access_token: String,
        server_adress: String,
        usage_data: Option<UsageData>,
    },
    Clear {
        usage_data: Option<UsageData>,
    },
    Custom(String),
}

/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_XML_Tags/HASP_CONFIG_XML_Tags.htm#Configur2>
fn usage_data_xml(usage_data: Option<UsageData>) -> String {
    match usage_data {
        Some(usage_data) => {
            let user_option_xml =
                usage_data
                    .user_option
                    .as_ref()
                    .map_or_else(String::new, |user_option| {
                        format!(
                            "<user_option>{}</user_option>",
                            match user_option {
                                UserOption::OptIn => "opt-in",
                                UserOption::OptOut => "opt-out",
                            }
                        )
                    });
            let app_name_xml = usage_data.app_name.map_or_else(String::new, |name| {
                format!("<app_name>{}</app_name>", escape_str_pcdata(&name))
            });
            let app_version_xml = usage_data.app_version.map_or_else(String::new, |version| {
                format!("<app_version>{}</app_version>", escape_str_pcdata(&version))
            });

            format!(
                "<usage_data_config>
            {}
            {}
            {}
            </usage_data_config>",
                user_option_xml, app_name_xml, app_version_xml,
            )
        }
        _ => String::new(),
    }
}

/// Generate a simple tag from a tag and value
/// If value is None, an empty string is returned
fn option_to_tag(tag: impl AsRef<str>, value: Option<impl AsRef<str>>) -> String {
    value.map_or(String::new(), |value| {
        format!(
            "<{}>{}</{}>",
            tag.as_ref(),
            escape_str_pcdata(value.as_ref()),
            tag.as_ref()
        )
    })
}

impl HaspConfigConfig {
    fn to_xml(self) -> String {
        match self {
            HaspConfigConfig::MachineAccount {
                identity_string,
                clear,
                usage_data,
            } => {
                let clear_str = if clear { "<clear/>" } else { "" };
                let identity_str = option_to_tag("identity", identity_string);
                format!(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>
                <haspconfig>
                    <credentials>
                        {}{}
                    </credentials>
                    {}
                </haspconfig>",
                    clear_str,
                    identity_str,
                    usage_data_xml(usage_data)
                )
            }
            HaspConfigConfig::ServerCertificate {
                certificate,
                certificate_store,
                usage_data,
                ..
            } => {
                format!(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>
                <haspconfig>
                    <server_config>
                        {}{}
                    </server_config>
                    {}
                </haspconfig>",
                    option_to_tag("certificate", certificate.into()),
                    option_to_tag("certificate_store", certificate_store),
                    usage_data_xml(usage_data)
                )
            }
            HaspConfigConfig::AuthRT {
                enabled,
                usage_data,
            } => {
                // Implementation for AuthRT
                format!(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>
                <haspconfig>
                    <auth_config>
                    <hasp_auth_rte>{}</hasp_auth_rte>
                    </auth_config>
                    {}
                </haspconfig>",
                    u8::from(enabled),
                    usage_data_xml(usage_data)
                )
            }
            HaspConfigConfig::JWT {
                access_token,
                server_adress,
                usage_data,
            } => {
                format!(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>
                <haspconfig>
                    <credentials>
                        <clear/>
                        <jwt_token>{}</jwt_token>
                    </credentials>
                    {}
                </haspconfig>",
                    option_to_tag(
                        "jwt_token",
                        format!("{}@{}", access_token, server_adress).into()
                    ),
                    usage_data_xml(usage_data),
                )
            }
            HaspConfigConfig::Clear { usage_data } => {
                format!(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>
                <haspconfig>
                    <credentials>
                    <clear/>
                    </credentials>
                    {}
                </haspconfig>",
                    usage_data_xml(usage_data)
                )
            }
            HaspConfigConfig::Custom(xml) => xml.to_string(),
        }
    }
}

impl From<&str> for HaspConfigConfig {
    fn from(s: &str) -> Self {
        HaspConfigConfig::Custom(s.to_string())
    }
}
impl From<String> for HaspConfigConfig {
    fn from(s: String) -> Self {
        HaspConfigConfig::Custom(s)
    }
}
/// Configures the behavior of the Licensing API for a specific application run-time session.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_config.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_config`]
///
pub fn hasp_config<T>(config: T, vendor_code: impl AsRef<str>) -> Result<(), HaspError>
where
    T: Into<HaspConfigConfig>,
{
    let config = config.into();
    let vendor_code = CString::new(vendor_code.as_ref()).map_err(|_| HaspStatus::HASP_INV_VCODE)?;
    let config_xml = CString::new(config.to_xml()).map_err(|_| HaspStatus::HASP_INV_FORMAT)?;
    println!("config_xml: {}", config_xml.to_string_lossy());
    unsafe {
        let result = hasp_api_ffi::hasp_config(
            config_xml.as_ptr(),
            vendor_code.as_ptr() as hasp_vendor_code_t,
        );
        match result {
            HaspStatus::HASP_STATUS_OK => Ok(()),
            _ => Err(result.into()),
        }
    }
}

/// Retrieves the version and build number of the Sentinel Licensing API library.
///
/// <https://docs.sentinel.thalesgroup.com/ldk/LDKdocs/API-licensing/Licensing_API/hasp_get_version.htm>
///
/// See also: [`crate::hasp_api_ffi::hasp_get_version`]
///
pub fn hasp_get_version(vendor_code: impl AsRef<str>) -> Result<HaspVersion, HaspError> {
    let vendor_code = CString::new(vendor_code.as_ref()).map_err(|_| HaspStatus::HASP_INV_VCODE)?;
    let mut version = HaspVersion {
        major_version: 0,
        minor_version: 0,
        generation_version: 0,
        build_number: 0,
    };
    unsafe {
        let result = hasp_api_ffi::hasp_get_version(
            &mut version.major_version,
            &mut version.minor_version,
            &mut version.generation_version,
            &mut version.build_number,
            vendor_code.as_ptr() as hasp_api_ffi::hasp_vendor_code_t,
        );

        match result {
            HaspStatus::HASP_STATUS_OK => Ok(version),
            _ => Err(result.into()),
        }
    }
}
