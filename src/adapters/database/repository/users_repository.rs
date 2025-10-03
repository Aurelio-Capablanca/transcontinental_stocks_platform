use tokio_postgres::Client;

use crate::{
    adapters::general::general_responses::{GeneralResponses, StopOperations},
    business::data_structures::platform_structs::{LoginUser, UsersAll},
};

pub async fn get_password_by_email(
    db_client: &Client,
    credentials: &LoginUser,
) -> Result<String, StopOperations> {
    let result = db_client
        .query(
            "select user_password from dev_test.users u where u.user_email = $1",
            &[&credentials.user_email],
        )
        .await
        .map_err(|err| StopOperations::from(err))?;
    let mut password: String = String::new();
    for row in result {
        password = row.get(0);
    }
    Ok(password)
}

pub async fn insert_user_repository(
    db_client: &Client,
    users: UsersAll,
) -> Result<GeneralResponses<String>, StopOperations> {
    let row = db_client
        .query_one(
            "INSERT INTO dev_test.users
        (user_name, user_lastname, user_phone, user_email, user_password)
        VALUES($1, $2, $3, $4, $5)
        RETURNING *;",
            &[
                &users.user_name.to_string(),
                &users.user_lastname.to_string(),
                &users.user_phone.to_string(),
                &users.user_email.to_string(),
                &users.user_password.to_string(),
            ],
        )
        .await
        .map_err(|err| StopOperations::from(err));

    let id = row.as_ref().unwrap().get::<_, i32>(0);
    let name = row.as_ref().unwrap().get::<_, String>(1);
    let lastname = row.as_ref().unwrap().get::<_, String>(2);
    let phone = row.as_ref().unwrap().get::<_, String>(3);
    let email = row.as_ref().unwrap().get::<_, String>(4);

    let res = format!(
        "Users = id : {:?}, name : {:?}, lastname : {:?}, phone : {:?}, email : {:?}",
        id, name, lastname, phone, email
    );
    Ok(GeneralResponses {
        message: Some("User Created!".to_string()),
        dataset: Some(res),
        code: Some(axum::http::StatusCode::OK.to_string()),
        error: Some("".to_string()),
    })
}
