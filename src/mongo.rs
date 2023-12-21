use bson::{doc, Document};
use mongodb::*;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use tokio;
use crate::helper::*;

const MONGODB_URI: &str = "mongodb://localhost:27017";
pub struct MongoClient {
    client: Client,
}

#[derive(Debug)]
enum MongoError {
    NotDirectoryError,
    NotFileError,
    FileNameError(String),
    NoSuchUser,
    NoSuchId,
    NoSuchPaper,
}

impl std::fmt::Display for MongoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MongoError::FileNameError(file) => {
                write!(f, "can resolve file name : {}", file)
            }
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}

impl Error for MongoError {}

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

    pub async fn insert_questions(&self, dir_path: &str) -> Result<bool, Box<dyn Error>> {
        // init
        let mut question_map = HashMap::<u64, Question>::new();
        let mut paper = Paper::default();

        let path = Path::new(dir_path);
        let paper_name = path.file_name().unwrap().to_str().unwrap();

        // access database
        let question_db = self.client.database("test");

        // init paper
        paper.date = paper_name.split("_").nth(0).unwrap().parse().unwrap();
        paper.paper_name = paper_name.to_string();
        let paper_cnt = question_db
            .collection::<Document>("papers")
            .estimated_document_count(None)
            .await
            .unwrap_or(0);
        paper.id = paper_cnt.into();

        // init question
        let question_coll = question_db.collection::<Question>("questions");
        let cnt = question_coll
            .estimated_document_count(None)
            .await
            .unwrap_or(0);

        if !path.is_dir() {
            return Err(Box::new(MongoError::NotDirectoryError));
        }
        for (_idx, entry) in path.read_dir().expect("read_dir call failed").enumerate() {
            let entry = entry.expect("read_dir entry failed");
            let file_path = entry.path();
            if !file_path.is_file() {
                return Err(Box::new(MongoError::NotFileError));
            }
            // let id = cnt + idx as u64;
            let url = file_path.to_str().unwrap();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let res: Vec<&str> = file_name.split("_").into_iter().collect();
            if let [QA, _date, question_number, question_tags, ..] = res.as_slice() {
                let q_no: u64 = question_number.split("-").nth(0).unwrap().parse().unwrap();
                let abs_q_no = q_no + cnt;
                paper.question_map.insert(q_no, abs_q_no);
                if question_map.contains_key(&abs_q_no) {
                    if QA == &"Q" {
                        match question_map.get_mut(&abs_q_no).unwrap().q_url {
                            Some(ref mut v) => {
                                v.push(url.to_string());
                            }
                            None => {
                                question_map.get_mut(&abs_q_no).unwrap().q_url =
                                    Some(vec![url.to_string()]);
                            }
                        }
                    } else {
                        match question_map.get_mut(&abs_q_no).unwrap().qa_url {
                            Some(ref mut v) => {
                                v.push(url.to_string());
                            }
                            None => {
                                question_map.get_mut(&abs_q_no).unwrap().qa_url =
                                    Some(vec![url.to_string()]);
                            }
                        }
                    }
                } else {
                    if QA == &"Q" {
                        question_map.insert(
                            abs_q_no,
                            Question {
                                id: abs_q_no,
                                q_url: Some(vec![url.to_string()]),
                                qa_url: None,
                                short_answer: None,
                                tags: question_tags.to_string(),
                            },
                        );
                    } else {
                        question_map.insert(
                            abs_q_no,
                            Question {
                                id: abs_q_no,
                                q_url: None,
                                qa_url: Some(vec![url.to_string()]),
                                short_answer: None,
                                tags: question_tags.to_string(),
                            },
                        );
                    }
                }
            } else {
                return Err(Box::new(MongoError::FileNameError(
                    file_path.to_str().unwrap().to_string(),
                )));
            }
        }

        //insert questions collections
        for (_, question) in question_map {
            match question_coll.insert_one(question, None).await {
                Ok(_) => {}
                Err(e) => {
                    println!("error, {e}");
                }
            }
        }

        //insert papers collections
        
        match question_db
            .collection::<Paper>("papers")
            .insert_one(paper.clone(), None)
            .await
        {
            Ok(_) => {
                println!("insert paper success");
            }
            Err(e) => {
                println!("{:?}", &paper);
                println!("insert paper failed : {:?}", e);
            }
        }

        Ok(true)
    }

    pub async fn insert_users(&self, user: User) -> Result<bool, Box<dyn Error>> {
        let question_db = self.client.database("test");
        let user_coll = question_db.collection::<User>("user");
        match user_coll.insert_one(user, None).await {
            Ok(_) => println!("insert user collection success"),
            Err(e) => println!("insert user collection failed : {}", e),
        }
        Ok(true)
    }

    pub async fn get_wrong_answer_list(
        &self,
        name: String,
        paper_name: String,
    ) -> Result<Vec<u64>, Box<dyn Error>> {
        let question_db = self.client.database("test");
        let user_coll = question_db.collection::<User>("user");
        let student_paper = user_coll
            .find_one(doc! {"name": &name, "paper_name": &paper_name}, None)
            .await?;
        if student_paper.is_none() {
            return Err(Box::new(MongoError::NoSuchPaper));
        }
        let ui = student_paper.unwrap();
        Ok(ui.wrong_question_list)
    }
}

mod test {
    use super::*;
    #[tokio::test]
    async fn test_mongo() {
        let client = MongoClient::new().await.ok().unwrap();
        let db_list = client.show_dbs().await.ok().unwrap();
        println!("{:?}", db_list);
    }

    #[tokio::test]
    async fn test_insert_questions() {
        let client = MongoClient::new().await.ok().unwrap();
        match client
            .insert_questions(
                "/home/yaoxiao/mydoc/rust-mongo/static/20231216/20231216_七年级培优班",
            )
            .await
        {
            Ok(_) => {
                println!("OK!");
            }
            Err(e) => {
                println!("error : {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_clear_collection() {
        let client = MongoClient::new().await.ok().unwrap();
        let test = client.client.database("test");
        let question_coll = test.collection::<Document>("questions");
        match question_coll.delete_many(doc! {}, None).await {
            Ok(_) => {
                println!("success");
            }
            _ => {
                println!("error!");
            }
        }
    }

    #[tokio::test]
    async fn test_insert_users() {
        let client = MongoClient::new().await.ok().unwrap();
        let user = User {
            name: "aaa".to_string(),
            id: 1,
            date: 20231216,
            paper_name: "20231216_七年级培优班".to_string(),
            wrong_question_list: vec![6, 7, 8],
            homework_question_list: vec![1, 2, 3, 4],
        };
        match client.insert_users(user).await {
            Ok(_) => {
                println!("OK!");
            }
            Err(e) => {
                println!("error : {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_wrong_answer_list() {
        let client = MongoClient::new().await.ok().unwrap();
        let name = "aaa".to_string();
        let paper_name = "20231216_七年级培优班".to_string();
        match client.get_wrong_answer_list(name, paper_name).await {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("error : {:?}, {}", e, "test_get_wrong_answer_list"),
        }
    }
}
