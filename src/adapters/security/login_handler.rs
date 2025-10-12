use std::sync::Arc;

use axum::{
    Json,
    extract::State,
   //handler::Handler,
    http::{/*self,*/ HeaderValue, Response},
};
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
) -> Result<Response<String>, StopOperations> {
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
        let general_response = serde_json::to_string(
            &GeneralResponses {
            message: Some("Login Successful".to_string()),
            dataset: Some(String::new()),
            code: Some(axum::http::StatusCode::OK.to_string()),
            error: Some("".to_string()),
        }
        ).unwrap();
        Ok(Response::builder()
            .status(axum::http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .header(
                "Set-Cookie",
                HeaderValue::from_str(
                    "session_token=tester-cookie; Path=/; HttpOnly; SameSite=Lax",
                )
                .unwrap(),
            )
            .body(general_response.into())
            .unwrap())
    }
}
