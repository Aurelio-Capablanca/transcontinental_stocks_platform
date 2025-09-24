use std::sync::Arc;

use bson::doc;
use mongodb::bson::Document;
use crate::adapters::database::db_pool;

mod adapters;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = db_pool::create_mongo_connection().await?;
    let state = db_pool::MongoClient{client : Arc::new(client)};


    println!("*****************************************************");
    let test = &state.client.database("aibdb").collection::<Document>("test");
    let cursor = test.find(Document::new()).await?;

    // while let Some(res) = cursor.advance().await {
        
    // }

    //println!("Collection : {:?}",*test);

    Ok(())
}
