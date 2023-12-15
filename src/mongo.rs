use chrono::Local;
use mongodb::*;
use bson::doc;
use std::error::Error;
use tokio;

const MONGODB_URI: &str = "mongodb://localhost:27017";
pub struct MongoClient {
    client: Client,
}

impl MongoClient {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let client_options = options::ClientOptions::parse(MONGODB_URI).await?;

        // Set any additional options if needed
        // client_options.app_name = Some("YourAppName".to_string());

        // Connect to the MongoDB server
        let client = Client::with_options(client_options)?;
        Ok(Self { client })
    }

    pub async fn show_dbs(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let db_list = self.client.list_database_names(doc! {}, None).await?;
        Ok(db_list)
    }


}
