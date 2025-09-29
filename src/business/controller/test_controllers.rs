use std::sync::Arc;

use axum::extract::State;
use serde::Serialize;
use tokio_postgres::GenericClient;
use crate::business::data_structures::platform_structs::ApplicationState;
use crate::adapters::{
    general::general_responses::{GeneralResponses, StopOperations},
};

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
pub struct TestSql {
    id: i32,
    constellation: String,
}

pub async fn test_sql(
    State(status): State<Arc<ApplicationState>>,
) -> Result<GeneralResponses<Vec<TestSql>>, StopOperations> {
    let client = status.database_postgres.client();

    let constellation = "Wyvern".to_string();
    let number_rows = client
        .execute(
            "insert into test_data (strings) values ($1)",
            &[&constellation],
        )
        .await
        .unwrap();
    println!("executed : {:?}", number_rows);
    let rows = client
        .query("select * from test_data td", &[])
        .await
        .map_err(|err| StopOperations::from(err))?;

    let mut response: Vec<TestSql> = Vec::new();
    for r in &rows {
        let id: i32 = r.get(0);
        let value: String = r.get(1);
        println!("id={:?}, col={:?}", &id, &value);
        response.push(TestSql {
            id: id,
            constellation: value,
        });
    }
    Ok(GeneralResponses {
        message: Some("Successful Connection".to_string()),
        dataset: Some(response),
        code: Some(axum::http::StatusCode::OK.to_string()),
        error: Some("None".to_string()),
    })
}
