use marine_sqlite_connector;
use marine_sqlite_connector::{Connection};
use crate::BTreeMap;
use serde_json::Value;
use chrono::NaiveDate;



pub struct Sqlite {
    pub connection: Connection
}

impl Sqlite {
    
    pub fn new() -> Sqlite {

        Sqlite {
            connection: marine_sqlite_connector::open(":memory:").unwrap()
        }
    }

    pub fn table(self: &Self, name: &str) {

        self.connection
            .execute("CREATE TABLE content (hash TEXT, post_type TEXT, date DATE, data TEXT);")
            .unwrap();
    }

    pub fn query(self: &Self, query: &String) -> Vec<BTreeMap<String, Value>> {

        let mut tables = self.connection.prepare(query).unwrap().cursor();

        let mut array: Vec<BTreeMap<String, Value>> = vec!();

        while let Some(row) = tables.next().unwrap() {
    
            let mut btreemap: BTreeMap<String, Value> = BTreeMap::new();

            // index 3 refers to data column! 
            let parsed_string: Value = serde_json::from_str(row[3].as_string().unwrap()).unwrap();
    
            if let Value::Object(map) = parsed_string {
                for (_key, value) in map {
                    btreemap.insert(_key, value);
                }
            }

            array.push(btreemap);
        }

        array
        
    }



}