use deadpool_postgres::Object;
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};

pub struct ApplicationState {
    pub database_postgres: Object,
    pub database_redis: MultiplexedConnection,
}

/*Users */

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub user_name: String,
    pub user_lastname: String,
    pub user_phone: String,
    pub user_email: String,
    pub user_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersAll {
    pub id_user: i64,
    pub user_name: String,
    pub user_lastname: String,
    pub user_phone: String,
    pub user_email: String,
    pub user_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub user_email: String,
    pub user_password: String,
}
/*Users */