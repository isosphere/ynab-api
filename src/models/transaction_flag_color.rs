/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.ynab.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionFlagColor : The transaction flag

/// The transaction flag
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionFlagColor {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "null")]
    Null,

}

impl ToString for TransactionFlagColor {
    fn to_string(&self) -> String {
        match self {
            Self::Red => String::from("red"),
            Self::Orange => String::from("orange"),
            Self::Yellow => String::from("yellow"),
            Self::Green => String::from("green"),
            Self::Blue => String::from("blue"),
            Self::Purple => String::from("purple"),
            Self::Null => String::from("null"),
        }
    }
}

impl Default for TransactionFlagColor {
    fn default() -> TransactionFlagColor {
        Self::Red
    }
}




