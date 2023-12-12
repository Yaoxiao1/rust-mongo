use mongodb::{Client, options::{ClientOptions}};
use std::error::Error;
use tokio;
use chrono::{TimeZone, Utc};
use mongodb::bson::doc;


const MONGODB_URI:&str = "mongodb://localhost:27017";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_options = ClientOptions::parse(MONGODB_URI).await?;
    
    // Set any additional options if needed
    // client_options.app_name = Some("YourAppName".to_string());
    
    // Connect to the MongoDB server
    let client = Client::with_options(client_options)?;
    
    // Access a specific database
    let questions = client.database("test").collection("questions");
    
    // Perform operations on the database   
    println!("connected!!");
    let new_doc = doc! {
        "title": "Parasite",
        "year": 2020,
        "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
        "released": Utc::now(),
     };
    let insert_result = questions.insert_one(new_doc.clone(), None).await?;
    println!("New document ID: {}", insert_result.inserted_id);
    
    Ok(())
}