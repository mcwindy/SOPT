pub mod invitation;
pub mod torrent_info;
pub mod user;
pub mod user_info;
// pub mod torrent;

use serde::Serialize;

/// General Response structure used to
/// communicate with frontend.
///
/// it contains:
/// 1. data, returned data or null
/// 2. success, the status of request
/// 3. errMsg, not so severe errors prompt
#[derive(Serialize, Debug)]
pub struct GeneralResponse {
    data: serde_json::Value,
    success: bool,
    #[serde(rename = "errMsg")]
    err_msg: String,
}

impl GeneralResponse {
    /// takes an error &str and return a `GeneralResponse` struct
    pub fn from_err(err_msg: &str) -> Self {
        GeneralResponse {
            data: serde_json::from_str("null").unwrap(),
            success: false,
            err_msg: String::from(err_msg),
        }
    }
}

impl Default for GeneralResponse {
    /// default success value with data is `null`
    fn default() -> Self {
        GeneralResponse {
            data: serde_json::from_str("null").unwrap(),
            success: true,
            err_msg: String::from(""),
        }
    }
}

/// A trait used to automated Json Response constructions.
/// It demands the type implemented `Serialize` trait.
pub trait ToResponse: Serialize {
    /// common wrapper for data.
    /// use serde_json to serialize into a `GeneralResponse` struct
    fn to_json(&self) -> GeneralResponse {
        let json_val = serde_json::to_value(self)
            // never happens
            .expect("unable to parse to json");
        GeneralResponse {
            data: json_val,
            success: true,
            err_msg: "".to_string(),
        }
    }
}
