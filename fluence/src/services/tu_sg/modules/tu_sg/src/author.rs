use std::collections::HashMap;
use serde_json::{json,Value};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Author {
    pub owner: String, // reference to identity: account, domain name, did 
    pub repository: String, // where content can be fetched
    pub content_mappings: HashMap<String,HashMap<String,Vec<String>>> // to map types and fields from author system to the sg
}

impl Author {

    pub fn json() -> Author {

        let data = r#"
        {
            "owner": "joera.p2p-citizen.eth",
            "repository": "nieuwsarchief.json",
            "content_mappings": {
                "home" : {
                    "title": ["title"]
                },
                "post": {
                    "title":["title"],
                    "content":["content"],
                    "date": ["date"],
                    "thumbnail": ["attachments","thumbnail"],
                    "year":["year"]
                }
            }
        }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        serde_json::from_value(v).unwrap()
    }
}