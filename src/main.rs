mod adapters;
mod business;


use crate::{adapters::database::db_pool, /*adapters::general ,*/};
use crate::business::controller::{test_controllers, user_controllers};
use axum::routing::post;
use axum::{Router, 
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    routing::get};
use std::sync::Arc;
use tower_http::cors::CorsLayer;




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Arc::new(db_pool::ApplicationState {
        database: db_pool::create_postgres_pool().await?,
    });
    

    let cors = CorsLayer::new()
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, axum::http::header::COOKIE]);

    let app = Router::new()
        .route("/", get(test_controllers::hello_world))
        .route("/test-sql", get(test_controllers::test_sql))
        .route("/users/create-user", post(user_controllers::create_first_user))
        .with_state(manager)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9088").await.unwrap();
    print!("Transcontinental Stocks is Alive at Localhost::9088!!!");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

