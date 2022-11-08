use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;
