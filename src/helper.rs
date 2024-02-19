use chrono::Local;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::collections::HashMap;


type Mydate = u64;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: u64,
    pub q_url: Option<Vec<String>>,
    pub qa_url: Option<Vec<String>>,
    pub short_answer: Option<Vec<String>>,
    pub tags: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Paper {
    pub id: u64,
    pub date: Mydate,
    pub paper_name: String,
    #[serde(serialize_with = "serialize_question_map", deserialize_with = "deserialize_question_map")]
    pub question_map: HashMap<u64, u64>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub paper_name: String,
    pub date: Mydate,
    pub wrong_question_list: Vec<u64>,
    pub homework_question_list: Vec<u64>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: u64,
    pub name: String,
}


#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct CreatePaperForm {
    // Define the fields of your form data structure
    pub user_name: String,
    pub paper_name: String,
    pub wrong_answer_list: Vec<u64>,
    pub homework_list: Vec<u64>
    // Add more fields as needed
}


#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UpdateUserForm {
    // Define the fields of your form data structure
    pub user_name: String,
    pub paper_name: String,
    pub wrong_answer_list: Vec<u64>,
    pub homework_list: Vec<u64>
    // Add more fields as needed
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SearchUserForm {
    // Define the fields of your form data structure
    pub user_name: String,
    pub paper_name: String,
    // Add more fields as needed
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PaperPath(pub String);


fn serialize_question_map<S>(question_map: &HashMap<u64, u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let map: HashMap<String, u64> = question_map.iter().map(|(&k, &v)| (k.to_string(), v)).collect();
    map.serialize(serializer)
}


fn deserialize_question_map<'de, D>(deserializer: D) -> Result<HashMap<u64, u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<String, u64> = Deserialize::deserialize(deserializer)?;
    let question_map: HashMap<u64, u64> = map.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect();
    Ok(question_map)
}

pub fn get_date_u64() -> u64 {
    let current_date = Local::now().format("%Y%m%d").to_string();
    let parsed_date = current_date.parse::<u64>().unwrap();
    parsed_date
}


mod test {
    use super::*;
    #[test]
    fn test_get_date() {
        let date = get_date_u64();
        assert_eq!(date, 20231227);
        assert_ne!(date, 20231226);
    }
}