use std::collections::HashMap;
use crate::BTreeMap;
use serde_json::Value;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Template {
    pub reference: String, // name 
    pub file: String, // path to file .. or cid
}

#[derive(Debug)]
pub struct Author {
    pub reference: String, // reference to identity: account, domain name, did 
    pub repository: String, // where content can be fetched
    pub public_key: String, // to potentially decrypt encrypted content
    pub content_mappings: HashMap<String,HashMap<String,Vec<String>>> // to map types and fields from author system to the sg
}

#[derive(Debug)]
pub struct Publisher {
    pub reference: String, /// contract address
    pub domain: String, // where can published content be found
    pub location: String, // where/how is content persisted/stored ., could be a server, could be a distributed storage deal 
    pub templates: Vec<Template>
}

#[derive(Debug,Serialize)]
pub struct TemplateObject  {
    pub body: BTreeMap<String, Value>,
    pub collections: Vec<BTreeMap<String, Value>>,
    pub base_url: String,
    pub assets_url: String,
    pub render_env: String
}