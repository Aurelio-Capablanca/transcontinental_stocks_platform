use std::sync::Arc;

use axum::{Json, extract::State};
use crate::business::data_structures::platform_structs::ApplicationState;
use crate::{
    adapters::{        
        general::general_responses::{GeneralResponses, StopOperations},
    },
    business::{data_structures::platform_structs::Users, logic::users_business_logic},
};

pub async fn create_first_user(
    State(state): State<Arc<ApplicationState>>,
    Json(body): Json<Users>,
) -> Result<GeneralResponses<String>, StopOperations> {
    users_business_logic::sign_users(state, body).await
}