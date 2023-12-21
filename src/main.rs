use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use env_logger::Env;
use helper::CreatePaperForm;
use serde::Deserialize;
use serde::Serialize;
use tokio;
mod mongo;
mod helper;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(show_dbs)
            .service(web::resource("/api/submitForm").route(web::post().to(submit_form)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



#[get("/show-dbs")]
async fn show_dbs() -> Result<impl Responder> {
    let client = mongo::MongoClient::new().await.unwrap();
    log::info!("connected to databases");
    let db_list = client.show_dbs().await?;
    log::info!("get database list");
    Ok(web::Json(db_list))
}

async fn submit_form(data: web::Json<CreatePaperForm>) -> impl Responder {

    // Access the form data using `data.field1`, `data.field2`, etc.
    // Process the form data as needed

    // Create a list of strings
    let string_list: Vec<String> = vec![
        "String 1".to_string(),
        "String 2".to_string(),
        "String 3".to_string(),
    ];

    // Return the list of strings as the response
    HttpResponse::Ok().json(string_list)
}