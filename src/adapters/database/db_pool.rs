use std::sync::Arc;
use deadpool_postgres::{Config, Manager, ManagerConfig, Object, Pool, RecyclingMethod};
use tokio_postgres::{Client, GenericClient, NoTls};


pub async fn create_postgres_pool() -> Result<Object<>, Box<dyn std::error::Error>>{

    let mut configuration = Config::new();
    configuration.host = Some("localhost".to_string());
    configuration.user = Some("superuserp".to_string());
    configuration.password = Some("jkl555".to_string());
    configuration.dbname = Some("transcontinental_stocks".to_string());
    configuration.manager = Some(ManagerConfig {recycling_method : RecyclingMethod::Fast});
    
    // let (client, connection) = tokio_postgres::connect("host=localhost user=superuserp password=jkl555 dbname=transcontinental_stocks ", NoTls)
    // .await?;

    // tokio::spawn(async move {
    //     if let Err(e) = connection.await{
    //         eprint!("Connection error : {}", e);
    //     }
    // });
    let pool : Pool = configuration.create_pool(None, NoTls)?;
    let object_manager = pool.get().await?;
    // let client = object_manager.client();
    Ok(object_manager)
}