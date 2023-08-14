
use serde_json::{json,Value};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Collection {
    pub key: String,
    pub query: String // straight sql query for now 
}

#[derive(Debug, Clone, Deserialize)]
pub struct Template {
    pub reference: String, // name 
    pub file: String, // path to file .. or cid
    pub path: String,
    pub collections: Vec<Collection>,
}

#[derive(Debug, Deserialize)]
pub struct Publication {
    pub owner: String, /// contract address
    pub domain: String, // where can published content be found
    pub location: String, // where/how is content persisted/stored ., could be a server, could be a distributed storage deal 
    pub templates: Vec<Template>
}

impl Publication {

    pub fn json() -> Publication {

        let data = r#"
        {
            "owner": "joera.p2p-citizen.eth",
            "domain": "https://unamore.publikaan.nl",
            "location":"s3::/archief/unamore/website",
            "templates": [
                {
                    "reference": "home",
                    "file": "home.handlebars",
                    "path": "index.html",
                    "collections": [
                        {
                            "key": "posts",
                            "query": "SELECT * FROM content WHERE post_type = 'post'"
                        }
                    ]
                },
                {
                    "reference": "post",
                    "file": "post.handlebars",
                    "path": "{slug}.html",
                    "collections": [
                        {
                            "key": "posts",
                            "query": "SELECT * from content WHERE post_type = 'post' LIMIT 2"
                        }
                    ]
                }
            ]
        }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        serde_json::from_value(v).unwrap()
    }
}
