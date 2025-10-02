use redis::{aio::MultiplexedConnection, Client};
use crate::adapters::general::general_responses::StopOperations;


pub async fn connect_redis_client() -> Result<MultiplexedConnection, StopOperations>{
    let url = "redis://127.0.0.1:6379/";
    let client = Client::open(url).map_err(|err| StopOperations::from(err)).unwrap();
    let connection: Result<MultiplexedConnection, StopOperations> = client.get_multiplexed_async_connection().await.map_err(|err| StopOperations::from(err));

    Ok(connection.unwrap())
}