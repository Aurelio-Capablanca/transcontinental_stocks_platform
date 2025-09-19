use tokio_postgres::{Error, NoTls};
mod adapters;



#[tokio::main]
async fn main() -> Result<(), Error> {
    

    
    let (client, connection) = tokio_postgres::connect("host=localhost user=superuserp password=jkl555 dbname=transcontinental_stocks ", NoTls)
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await{
            eprint!("Connection error : {}", e);
        }
    });

    let constellation = "Sagittarius".to_string();
    let number_rows = client.execute("insert into test_data (strings) values ($1)", &[&constellation ]).await.unwrap();
    println!("executed : {:?}",number_rows);
    let rows = client.query("select * from test_data td", &[]).await.unwrap();
    for r in rows {
        let id : i32 = r.get(0);
        let value : String = r.get(1);
        println!("id={:?}, col={:?}",id,value);
    }

    Ok(())
}
