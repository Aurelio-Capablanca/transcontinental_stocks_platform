use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    adapters::{
        database::db_pool::ApplicationState,
        general::general_responses::{GeneralResponses, StopOperations},
    },
    business::{data_structures::platform_structs::Users, logic::users_business_logic},
};


pub async fn create_first_user(
    State(state): State<Arc<ApplicationState>>,
    Json(body): Json<Users>,
) -> Result<GeneralResponses<String>, StopOperations> {
    users_business_logic::create_users(state, body).await
}
