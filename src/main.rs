mod adapters;
mod business;

use tokio_postgres::GenericClient;
use crate::adapters::database::db_pool;
use std::sync::Arc;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let manager = Arc::new(db_pool::create_postgres_pool().await?);
    let client = manager.client();

    let constellation = "Odin".to_string();
    let number_rows = client.execute("insert into test_data (strings) values ($1)", &[&constellation]).await.unwrap();
    println!("executed : {:?}",number_rows);
    let rows = client.query("select * from test_data td", &[]).await.unwrap();
    
    for r in rows {
        let id : i32 = r.get(0);
        let value : String = r.get(1);
        println!("id={:?}, col={:?}",id,value);
    }

    Ok(())
}
