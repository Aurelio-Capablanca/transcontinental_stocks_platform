

use deadpool_postgres::Object;
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};


pub struct ApplicationState {
    pub database_postgres: Object,
    pub database_redis: MultiplexedConnection
}


/*
id_user 
user_name
user_lastname
user_phone
user_email
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub id_user: i64,
    pub user_name: String,
    pub user_lastname: String,
    pub user_phone: String,
    pub user_email: String,
}
