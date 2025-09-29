use deadpool_postgres::{Config, ManagerConfig, Object, Pool, RecyclingMethod};
use tokio_postgres::{NoTls};

//use crate::adapters::general::general_responses::StopOperations;



pub async fn create_postgres_pool() -> Result<Object<>, Box<dyn std::error::Error>>{

    let mut configuration = Config::new();
    configuration.host = Some("localhost".to_string());
    configuration.user = Some("superuserp".to_string());
    configuration.password = Some("jkl555".to_string());
    configuration.dbname = Some("transcontinental_stocks".to_string());
    configuration.manager = Some(ManagerConfig {recycling_method : RecyclingMethod::Fast});
    
    let pool : Pool = configuration
    .create_pool(None, NoTls)?;

    let object_manager = pool.get().await?;
  
    Ok(object_manager)
}