use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::statistics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Statistics {
    pub date: DateTime<Utc>,
    pub data: serde_json::Value,
}
