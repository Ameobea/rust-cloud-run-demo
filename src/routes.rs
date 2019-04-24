use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::models::{InsertablePageView, PageView};
use crate::schema;
use crate::DbConn;

#[get("/")]
pub fn index() -> &'static str {
    "Application successfully started!"
}

#[post("/page_view", data = "<page_view>")]
pub fn create_page_view(
    conn: DbConn,
    page_view: Json<InsertablePageView>,
) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::pageviews::table)
        .values(&page_view.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/page_view")]
pub fn list_page_views(conn: DbConn) -> Result<Json<Vec<PageView>>, String> {
    use crate::schema::pageviews::dsl::*;

    pageviews
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying page views: {:?}", err);
            "Error querying page views from the database".into()
        })
        .map(Json)
}
