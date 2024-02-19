use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use env_logger::Env;
use helper::{CreatePaperForm, UpdateUserForm, SearchUserForm, PaperPath};
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
            .route("/api/submitForm", web::post().to(submit_form))
            .route("/api/updateUser", web::post().to(update_user))
            .route("/api/searchUser", web::post().to(search_user))
            .route("/api/insertQuestions", web::post().to(insert_paper_and_questions))
            .route("/api/insertUserInfo", web::post().to(insert_user_info))
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

    println!("submit_from : {:?}", &data);
    let form = data.into_inner();
    let mongo_client = mongo::MongoClient::new().await.unwrap();
    let userid = mongo_client.find_user_id(form.user_name.clone()).await.unwrap();
    let user = helper::User {
        id: userid,
        name: form.user_name.clone(),
        paper_name: form.paper_name.clone(),
        date: helper::get_date_u64(),
        wrong_question_list: form.wrong_answer_list,
        homework_question_list: form.homework_list
    };
    match mongo_client.insert_users(user).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("Error: {e}");
        }
    }
    let lst = match mongo_client.get_wrong_answer_list(form.user_name.clone(), form.paper_name.clone()).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error: {e}");
            vec![]
        }
    };
    // Return the list of strings as the response
    HttpResponse::Ok().json(lst)
}

async fn update_user(data: web::Json<UpdateUserForm>) -> impl Responder {

    // Access the form data using `data.field1`, `data.field2`, etc.
    // Process the form data as needed

    println!("submit_from : {:?}", &data);
    let form = data.into_inner();
    let mongo_client = mongo::MongoClient::new().await.unwrap();
    let userid = match mongo_client.find_user_id(form.user_name.clone()).await {
        Ok(id) => id,
        Err(e) => {
            log::error!("{e:?}");
            return HttpResponse::InternalServerError().body("username not found in database");
        }
    };
    let user = helper::User {
        id: userid,
        name: form.user_name.clone(),
        paper_name: form.paper_name.clone(),
        date: helper::get_date_u64(),
        wrong_question_list: form.wrong_answer_list,
        homework_question_list: form.homework_list
    };
    match mongo_client.insert_users(user).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("Error: {e}");
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    }
    // Return the list of strings as the response
    HttpResponse::Ok().json(())
}

async fn insert_user_info(data: web::Json<String>) -> impl Responder {
    log::info!("insert user info, data: {:?}", &data);
    let username = data.into_inner();
    let mongo_client = mongo::MongoClient::new().await.unwrap();
    match mongo_client.insert_user_info(username).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("Error: {e}");
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    }
    HttpResponse::Ok().json(())
}

async fn search_user(data: web::Json<SearchUserForm>) -> impl Responder {
    log::info!("submit_from : {:?}", &data);
    let form = data.into_inner();
    let mongo_client = mongo::MongoClient::new().await.unwrap();
    let lst = match mongo_client.get_wrong_answer_list(form.user_name.clone(), form.paper_name.clone()).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error: {e}");
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    };
    HttpResponse::Ok().json(lst)
}

async fn insert_paper_and_questions(data: web::Json<String>) -> impl Responder {
    log::info!("insert questions and papers, path: {:?}", &data);
    let path = data.into_inner();
    let mongo_client = mongo::MongoClient::new().await.unwrap();
    match mongo_client.insert_questions(&path).await {
        Ok(_) => {

        }
        Err(e) => {
            log::error!("Error: {e}");
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    }
    
    HttpResponse::Ok().json(())
}