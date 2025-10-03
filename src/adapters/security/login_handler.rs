use std::sync::Arc;

use axum::{Json, extract::State};
use tokio_postgres::GenericClient;

use crate::{
    adapters::{
        database::repository::users_repository::get_password_by_email,
        general::general_responses::{GeneralResponses, StopOperations},
        security::password_handler::verify_passwords,
    },
    business::data_structures::platform_structs::{ApplicationState, LoginUser},
};

pub async fn login_actions(
    State(state): State<Arc<ApplicationState>>,
    Json(credentials): Json<LoginUser>,
) -> Result<GeneralResponses<String>, StopOperations> {
    let db_client = state.database_postgres.client();
    let password_find = match get_password_by_email(db_client, &credentials).await {
        Ok(pass) => pass,
        Err(err) => {
            return Err(StopOperations::InternalMessage(format!(
                "Error From : {:?}",
                err
            )));
        }
    };
    let verification = match verify_passwords(&credentials.user_password, &password_find) {
        Ok(result) => result,
        Err(err) => {
            return Err(StopOperations::InternalMessage(format!(
                "Error From : {:?}",
                err
            )));
        }
    };
    if !verification {
        Err(StopOperations::InternalMessage(format!(
            "Login Failed ! ! !"
        )))
    } else {
        Ok(GeneralResponses {
            message: Some("Login Successful".to_string()),
            dataset: Some(String::new()),
            code: Some(axum::http::StatusCode::OK.to_string()),
            error: Some("".to_string()),
        })
    }
}
