mod adapters;
mod business;


use crate::{adapters::database::db_pool, /*adapters::general ,*/};
use crate::business::controller::test_controllers;
use axum::{Router, routing::get};
use std::sync::Arc;




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Arc::new(db_pool::ApplicationState {
        database: db_pool::create_postgres_pool().await?,
    });
    
    let app = Router::new()
        .route("/", get(test_controllers::hello_world))
        .route("/test-sql", get(test_controllers::test_sql))
        .with_state(manager);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9088").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    print!("Transcontinental Stocks is Alive at Localhost::9088!!!");
    Ok(())
}

