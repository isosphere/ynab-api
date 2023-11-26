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
pub struct CategoryGroupWithCategories {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// Whether or not the category group is hidden
    #[serde(rename = "hidden")]
    pub hidden: bool,
    /// Whether or not the category group has been deleted.  Deleted category groups will only be included in delta requests.
    #[serde(rename = "deleted")]
    pub deleted: bool,
    /// Category group categories.  Amounts (budgeted, activity, balance, etc.) are specific to the current budget month (UTC).
    #[serde(rename = "categories")]
    pub categories: Vec<crate::models::Category>,
}

impl CategoryGroupWithCategories {
    pub fn new(id: uuid::Uuid, name: String, hidden: bool, deleted: bool, categories: Vec<crate::models::Category>) -> CategoryGroupWithCategories {
        CategoryGroupWithCategories {
            id,
            name,
            hidden,
            deleted,
            categories,
        }
    }
}


