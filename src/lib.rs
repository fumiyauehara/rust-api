use std::env;
use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;
pub mod models;
pub mod schema;
pub mod api {
    pub mod handlers;
    pub mod models;
    pub mod usecases;
}
pub mod websocket {
    pub mod handlers;
}


pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DB_URL_FROM_APP")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
