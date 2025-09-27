use std::sync::Arc;

use axum::extract::State;
use tokio_postgres::GenericClient;

use crate::adapters::database::db_pool::DatabaseState;



pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn test_sql(State(status) : State<Arc<DatabaseState>>) {
    
    let client = status.database.client();
    
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
        .unwrap();

    for r in rows {
        let id: i32 = r.get(0);
        let value: String = r.get(1);
        println!("id={:?}, col={:?}", id, value);
    }
}