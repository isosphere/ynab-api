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
pub struct Category {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "category_group_id")]
    pub category_group_id: uuid::Uuid,
    #[serde(rename = "category_group_name", skip_serializing_if = "Option::is_none")]
    pub category_group_name: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    /// Whether or not the category is hidden
    #[serde(rename = "hidden")]
    pub hidden: bool,
    /// DEPRECATED: No longer used.  Value will always be null.
    #[serde(rename = "original_category_group_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub original_category_group_id: Option<Option<uuid::Uuid>>,
    #[serde(rename = "note", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub note: Option<Option<String>>,
    /// Budgeted amount in milliunits format
    #[serde(rename = "budgeted")]
    pub budgeted: i64,
    /// Activity amount in milliunits format
    #[serde(rename = "activity")]
    pub activity: i64,
    /// Balance in milliunits format
    #[serde(rename = "balance")]
    pub balance: i64,
    /// The type of goal, if the category has a goal (TB='Target Category Balance', TBD='Target Category Balance by Date', MF='Monthly Funding', NEED='Plan Your Spending')
    #[serde(rename = "goal_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_type: Option<Option<GoalType>>,
    /// A day offset modifier for the goal's due date. When goal_cadence is 2 (Weekly), this value specifies which day of the week the goal is due (0 = Sunday, 6 = Saturday). Otherwise, this value specifies which day of the month the goal is due (1 = 1st, 31 = 31st, null = Last day of Month).
    #[serde(rename = "goal_day", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_day: Option<Option<i32>>,
    /// The goal cadence. Value in range 0-14. There are two subsets of these values which behave differently. For values 0, 1, 2, and 13, the goal's due date repeats every goal_cadence * goal_cadence_frequency, where 0 = None, 1 = Monthly, 2 = Weekly, and 13 = Yearly. For example, goal_cadence 1 with goal_cadence_frequency 2 means the goal is due every other month. For values 3-12 and 14, goal_cadence_frequency is ignored and the goal's due date repeats every goal_cadence, where 3 = Every 2 Months, 4 = Every 3 Months, ..., 12 = Every 11 Months, and 14 = Every 2 Years.
    #[serde(rename = "goal_cadence", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_cadence: Option<Option<i32>>,
    /// The goal cadence frequency. When goal_cadence is 0, 1, 2, or 13, a goal's due date repeats every goal_cadence * goal_cadence_frequency. For example, goal_cadence 1 with goal_cadence_frequency 2 means the goal is due every other month.  When goal_cadence is 3-12 or 14, goal_cadence_frequency is ignored.
    #[serde(rename = "goal_cadence_frequency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_cadence_frequency: Option<Option<i32>>,
    /// The month a goal was created
    #[serde(rename = "goal_creation_month", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_creation_month: Option<Option<String>>,
    /// The goal target amount in milliunits
    #[serde(rename = "goal_target", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_target: Option<Option<i64>>,
    /// The original target month for the goal to be completed.  Only some goal types specify this date.
    #[serde(rename = "goal_target_month", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_target_month: Option<Option<String>>,
    /// The percentage completion of the goal
    #[serde(rename = "goal_percentage_complete", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_percentage_complete: Option<Option<i32>>,
    /// The number of months, including the current month, left in the current goal period.
    #[serde(rename = "goal_months_to_budget", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_months_to_budget: Option<Option<i32>>,
    /// The amount of funding still needed in the current month to stay on track towards completing the goal within the current goal period. This amount will generally correspond to the 'Underfunded' amount in the web and mobile clients except when viewing a category with a Needed for Spending Goal in a future month.  The web and mobile clients will ignore any funding from a prior goal period when viewing category with a Needed for Spending Goal in a future month.
    #[serde(rename = "goal_under_funded", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_under_funded: Option<Option<i64>>,
    /// The total amount funded towards the goal within the current goal period.
    #[serde(rename = "goal_overall_funded", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_overall_funded: Option<Option<i64>>,
    /// The amount of funding still needed to complete the goal within the current goal period.
    #[serde(rename = "goal_overall_left", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub goal_overall_left: Option<Option<i64>>,
    /// Whether or not the category has been deleted.  Deleted categories will only be included in delta requests.
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl Category {
    pub fn new(id: uuid::Uuid, category_group_id: uuid::Uuid, name: String, hidden: bool, budgeted: i64, activity: i64, balance: i64, deleted: bool) -> Category {
        Category {
            id,
            category_group_id,
            category_group_name: None,
            name,
            hidden,
            original_category_group_id: None,
            note: None,
            budgeted,
            activity,
            balance,
            goal_type: None,
            goal_day: None,
            goal_cadence: None,
            goal_cadence_frequency: None,
            goal_creation_month: None,
            goal_target: None,
            goal_target_month: None,
            goal_percentage_complete: None,
            goal_months_to_budget: None,
            goal_under_funded: None,
            goal_overall_funded: None,
            goal_overall_left: None,
            deleted,
        }
    }
}

/// The type of goal, if the category has a goal (TB='Target Category Balance', TBD='Target Category Balance by Date', MF='Monthly Funding', NEED='Plan Your Spending')
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GoalType {
    #[serde(rename = "TB")]
    Tb,
    #[serde(rename = "TBD")]
    Tbd,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "NEED")]
    Need,
    #[serde(rename = "DEBT")]
    Debt,
    #[serde(rename = "null")]
    Null,
}

impl Default for GoalType {
    fn default() -> GoalType {
        Self::Tb
    }
}

