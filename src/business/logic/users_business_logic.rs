use std::sync::Arc;

use crate::adapters::database::repository::users_repository::insert_user_repository;
use crate::adapters::security::password_handler::hash_password;
use crate::business::data_structures::platform_structs::{ApplicationState, UsersAll};
use crate::{
    adapters::general::general_responses::{GeneralResponses, StopOperations},
    business::data_structures::platform_structs::Users,
};
use tokio_postgres::GenericClient;

pub async fn sign_users(
    state: Arc<ApplicationState>,
    users: Users,
) -> Result<GeneralResponses<String>, StopOperations> {
    let hash_password = hash_password(&users.user_password);

    let password = match hash_password {
        Ok(res) => res,
        Err(err) => {
            return Result::Err(StopOperations::InternalMessage(format!("{}", err)));
        }
    };

    let db_client: &tokio_postgres::Client = state.database_postgres.client();
    let user_insert = UsersAll {
        id_user: 0,
        user_name: users.user_name,
        user_lastname: users.user_lastname,
        user_phone: users.user_phone,
        user_email: users.user_email,
        user_password: password,
    };
    insert_user_repository(db_client, user_insert).await
}
