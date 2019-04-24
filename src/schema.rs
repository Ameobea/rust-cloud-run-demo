table! {
    pageviews (id) {
        id -> Bigint,
        view_time -> Datetime,
        url -> Varchar,
        user_agent -> Varchar,
        referrer -> Varchar,
        device_type -> Tinyint,
    }
}
