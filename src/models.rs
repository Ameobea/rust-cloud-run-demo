use chrono::NaiveDateTime;

use crate::schema::pageviews;

/// This represents a page view pulled from the database, including the auto-generated fields
#[derive(Serialize, Deserialize, Queryable)]
pub struct PageView {
    pub id: i64,
    pub view_time: NaiveDateTime,
    pub url: String,
    pub user_agent: String,
    pub referrer: String,
    pub device_type: i8,
}

/// This represents a page view being inserted into the database, without the auto-generated fields
#[derive(Deserialize, Insertable)]
#[table_name = "pageviews"]
pub struct InsertablePageView {
    pub url: String,
    pub user_agent: String,
    pub referrer: String,
    pub device_type: i8,
}
