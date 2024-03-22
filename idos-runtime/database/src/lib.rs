pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct Client {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Client {
    pub fn new(dsn: &str) -> anyhow::Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(dsn);
        let pool = match Pool::builder().test_on_check_out(true).build(manager) {
            Ok(it) => it,
            Err(err) => anyhow::bail!("failed build connection pool: {}", err),
        };
        Ok(Self { pool })
    }
}
