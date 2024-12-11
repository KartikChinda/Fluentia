use mongodb::{options::ClientOptions, Client};
use std::env;

// this is initializing a connection with DB.
pub async fn init_db() -> mongodb::error::result<Client> {
    let uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string()); //||_| means that I dont care about whatevcer value comes in here. And that is true, because we are only returning a local value, so we dont care what error (that comes in this value) gets shown.

    let mut client_options = ClientOptions::parse(&uri).await?;
    // just setting the name that db will use to associate to my app.
    client_options.app_name = Some("Fluentia".to_string());

    let client = Client::with_options(client_options);
    Ok(client)
}
