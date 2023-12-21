use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use env_logger::Env;
use serde::Deserialize;
use serde::Serialize;
use tokio;
mod mongo;
mod helper;

#[get("/show-dbs")]
async fn show_dbs() -> Result<impl Responder> {
    let client = mongo::MongoClient::new().await.unwrap();
    log::info!("connected to databases");
    let db_list = client.show_dbs().await?;
    log::info!("get database list");
    Ok(web::Json(db_list))
}

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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
