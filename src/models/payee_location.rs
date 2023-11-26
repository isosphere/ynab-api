/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.ynab.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayeeLocation {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "payee_id")]
    pub payee_id: uuid::Uuid,
    #[serde(rename = "latitude")]
    pub latitude: String,
    #[serde(rename = "longitude")]
    pub longitude: String,
    /// Whether or not the payee location has been deleted.  Deleted payee locations will only be included in delta requests.
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl PayeeLocation {
    pub fn new(id: uuid::Uuid, payee_id: uuid::Uuid, latitude: String, longitude: String, deleted: bool) -> PayeeLocation {
        PayeeLocation {
            id,
            payee_id,
            latitude,
            longitude,
            deleted,
        }
    }
}


