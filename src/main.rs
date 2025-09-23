use mongodb::{
    Client,
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};

mod adapters;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let url = "mongodb://localhost:27017";
    let mut client_option = ClientOptions::parse(url).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_option.server_api = Some(server_api);
    let client = Client::with_options(client_option);
    let usage_client: Option<Client> = match client {
        Ok(client_get) => Some(client_get),
        Err(err) => {
            println!("Error for Mongodb client : {:?}", err);
            None
        }
    };
    let test = usage_client.unwrap()
        .database("test")
        .run_command(doc! {"ping":1})
        .await?;

     println!("Pinged your deployment. You successfully connected to MongoDB!");
     print!("document One : {:?}",test);
    Ok(())
}
