
use std::collections::HashMap;
use serde_json::{Map, Value, from_str, json, from_slice, from_value, };
use serde::{Serialize, Deserialize};
use std::collections::{ BTreeMap};
use multihash::Multihash;
use marine_sqlite_connector;
use marine_sqlite_connector::{Connection, State, Value as SqlValue};



#[derive(Debug, Serialize, Deserialize)]
pub struct Content  {
    pub hash: Option<String>,
    pub post_type: Option<String>,
    pub mapped: Option<BTreeMap<String, Value>>,
    pub mapped_str: Option<String>,
}

impl Content {

    pub fn new(raw: &Value, author: &crate::types::Author) -> Content {

        let mut mapped_content: Map<String, Value> = Map::new();
        let post_type = "post";
        let mapping: &HashMap<String,Vec<String>> = author.content_mappings.get(post_type).unwrap();

        for (mapping_key, mapping_value) in mapping.iter() {

            let mut content_value: &Value = &json!(null);

            if mapping_value.len() == 1 {
                content_value = &raw[&mapping_value[0]];
            } else if mapping_value.len() == 2 {
               // this is an json like array of json like objects ... 
               // how to quickly do a find? 
                let mut keys: Vec<&String> = vec!();
                if let Value::Array(array) = &raw[&mapping_value[0]] {
                    for o in array {
                        if let Some(object) = o.as_object() {
                            keys = object.keys().collect();  
                        } 
                    }
                }

                if let Some(index) = keys.iter().position(|&x| x == mapping_key) {
                    content_value = &raw[&mapping_value[0]][index][&mapping_value[1]];
                }
            }
                
            mapped_content.insert(
                mapping_key.to_string(),
                content_value.clone()
            );
        }

        let mapped_content_as_string = serde_json::to_string(&Value::Object(mapped_content.clone())).unwrap();
        let res = crate::ipfs_hash(&mapped_content_as_string, &String::from("http://localhost"));
        let mut hash: Option<String> = None;
        if res.output.len() > 0 {
            hash = Some(res.output[0].clone());
        }   

        Content {
            hash,
            post_type: Some(post_type.to_string()),
            mapped: Some(from_value::<BTreeMap<String, Value>>(Value::Object(mapped_content).clone()).unwrap()),
            mapped_str: Some(mapped_content_as_string)
        }
    }

    pub fn store(self: &mut Self, sqlite: &Connection)  {

        let mut statement = sqlite
            .prepare("INSERT INTO content (hash, post_type, data) VALUES (?,?,?);")
            .unwrap();

        statement.bind(1, &*self.hash.clone().unwrap()).unwrap();
        statement.bind(2, &*self.post_type.clone().unwrap()).unwrap();
        statement.bind(3, &*self.mapped_str.clone().unwrap()).unwrap();
    
        while let State::Row = statement.next().unwrap() {
            println!("halloo");
        }
       
        // store on sq lite -- replace w distributed db later 
    }

}  

pub fn create_table(sqlite: &Connection) {

    sqlite
        .execute(
            "
            CREATE TABLE content (hash TEXT, post_type TEXT, data TEXT);
        ",
        )
        .unwrap();
}

pub fn fetch(sqlite: &Connection) -> Vec<Content> {

    let mut array: Vec<Content> = vec!();

    let mut tables = sqlite
    .prepare(
        "SELECT * FROM content 
        WHERE post_type = 'post';
    ").unwrap().cursor();

    while let Some(row) = tables.next().unwrap() {

        let mut btreemap: BTreeMap<String, Value> = BTreeMap::new();

        let parsed_string: Value = serde_json::from_str(row[2].as_string().unwrap()).unwrap();

        if let Value::Object(map) = parsed_string {
            for (_key, value) in map {
                btreemap.insert(_key, value);
            }
        }
    
        let co = Content {
            hash : Some(String::from(row[0].as_string().unwrap())),
            post_type : Some(String::from(row[1].as_string().unwrap())),
            mapped: Some(btreemap),
            mapped_str: Some(String::from(row[2].as_string().unwrap()))
        };
       
        // hier al in struct parsen? 
        array.push(co);
    }

    array
}

pub fn get_posts(sqlite: &Connection) -> Vec<BTreeMap<String, Value>> {

    let mut array: Vec<BTreeMap<String, Value>> = vec!();

    let mut tables = sqlite
    .prepare(
        "SELECT * FROM content 
        WHERE post_type = 'post';
    ").unwrap().cursor();

    while let Some(row) = tables.next().unwrap() {

        let mut btreemap: BTreeMap<String, Value> = BTreeMap::new();

        let parsed_string: Value = serde_json::from_str(row[2].as_string().unwrap()).unwrap();

        if let Value::Object(map) = parsed_string {
            for (_key, value) in map {
                btreemap.insert(_key, value);
            }
        }
    
        // let co = Content {
        //     hash : Some(String::from(row[0].as_string().unwrap())),
        //     post_type : Some(String::from(row[1].as_string().unwrap())),
        //     mapped: Some(btreemap),
        //     mapped_str: Some(String::from(row[2].as_string().unwrap()))
        // };
       
        // hier al in struct parsen? 
        array.push(btreemap);
    }

    array
}

