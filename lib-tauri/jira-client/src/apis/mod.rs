use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod application_properties_api;
pub mod applicationrole_api;
pub mod attachment_api;
pub mod avatar_api;
pub mod backlog_api;
pub mod board_api;
pub mod cluster_api;
pub mod comment_api;
pub mod component_api;
pub mod configuration_api;
pub mod custom_field_option_api;
pub mod custom_fields_api;
pub mod dashboard_api;
pub mod email_templates_api;
pub mod epic_api;
pub mod field_api;
pub mod filter_api;
pub mod group_api;
pub mod groups_api;
pub mod groupuserpicker_api;
pub mod index_api;
pub mod index_snapshot_api;
pub mod issue_api;
pub mod issue_link_api;
pub mod issue_link_type_api;
pub mod issuesecurityschemes_api;
pub mod issuetype_api;
pub mod issuetypescheme_api;
pub mod jql_api;
pub mod license_validator_api;
pub mod monitoring_api;
pub mod mypermissions_api;
pub mod mypreferences_api;
pub mod myself_api;
pub mod notificationscheme_api;
pub mod password_api;
pub mod permissions_api;
pub mod permissionscheme_api;
pub mod priority_api;
pub mod priorityschemes_api;
pub mod project_api;
pub mod project_category_api;
pub mod projects_api;
pub mod projectvalidate_api;
pub mod reindex_api;
pub mod resolution_api;
pub mod role_api;
pub mod screens_api;
pub mod search_api;
pub mod securitylevel_api;
pub mod server_info_api;
pub mod session_api;
pub mod settings_api;
pub mod sprint_api;
pub mod status_api;
pub mod statuscategory_api;
pub mod terminology_api;
pub mod universal_avatar_api;
pub mod upgrade_api;
pub mod user_api;
pub mod version_api;
pub mod websudo_api;
pub mod workflow_api;
pub mod workflowscheme_api;
pub mod worklog_api;

pub mod configuration;
