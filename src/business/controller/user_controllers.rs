use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{adapters::{
    database::db_pool::ApplicationState,
    general::general_responses::{GeneralResponses, StopOperations},
}, business::data_structures::platform_structs::Users};

pub async fn create_first_user(State(state): State<Arc<ApplicationState>>, Json(body) :Json<Users>) -> Result<(), StopOperations> {
    Ok(())
}
