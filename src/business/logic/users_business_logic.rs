use std::sync::Arc;

use axum::{Json, extract::State};
use tokio_postgres::{GenericClient, Row};

use crate::{
    adapters::{
        database::db_pool::ApplicationState,
        general::general_responses::{GeneralResponses, StopOperations},
    },
    business::data_structures::platform_structs::Users,
};

pub async fn create_users(
    state: Arc<ApplicationState>,
    users: Users,
) -> Result<GeneralResponses<String>, StopOperations> {
    let db_client = state.database.client();
    let row = db_client
        .query_one(
            "INSERT INTO dev_test.users
        (user_name, user_lastname, user_phone, user_email)
        VALUES($1, $2, $3, $4)
        RETURNING *;",
            &[
                &users.user_name.to_string(),
                &users.user_lastname.to_string(),
                &users.user_phone.to_string(),
                &users.user_email.to_string(),
            ],
        )
        .await
        .map_err(|err| StopOperations::from(err));

    let id = row.as_ref().unwrap().get::<_, i32>(0);
    let name = row.as_ref().unwrap().get::<_, String>(1);
    let lastname = row.as_ref().unwrap().get::<_, String>(2);
    let phone = row.as_ref().unwrap().get::<_, String>(3);
    let email = row.as_ref().unwrap().get::<_, String>(4);

    let res = format!("Users = id : {:?}, name : {:?}, lastname : {:?}, phone : {:?}, email : {:?}", id, name,lastname,phone,email);
    Ok(GeneralResponses {
        message: Some("User Created!".to_string()),
        dataset: Some(res),
        code: Some(axum::http::StatusCode::OK.to_string()),
        error: Some("".to_string()),
    })
}
