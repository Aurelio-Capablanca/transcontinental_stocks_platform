/*
id_user 
user_name
user_lastname
user_phone
user_email
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub id_user: i64,
    pub user_name: String,
    pub user_lastname: String,
    pub user_phone: String,
    pub user_email: String,
}
