#![allow(unused)]

use std::collections::HashMap;
use serde_json::{Value, from_slice, from_value, json};
use crate::BTreeMap;

pub fn render (author: String, publisher: String, content: String) {

    let sqlite = crate::sqlite::Sqlite::new();
    sqlite.table("content");
    let mut renderer = crate::renderer::Renderer::new(&sqlite);
    renderer.init();

    let author: crate::author::Author = crate::author::Author::json();
    let publication: crate::publication::Publication = crate::publication::Publication::json();

    let res = crate::read(&author.repository,"/content/");

    for e in res.errors.iter() {
        println!("{:?}", e);
    }

    if res.output.len() == 1 {
        
        let raw: Value = from_slice(&res.output[0]).unwrap();

        if let Some(post_array) = raw["posts"].as_array() {
            for post in post_array {
                let mut content = crate::Content::new("post", &post, &author);                
                content.store(&sqlite);
                renderer.render_template(&publication, content);
            }
        }

        let post = json!({
            "title":"l'Ouverture d'Unamore",
            "post_type": "home"
        });

        let mut content = crate::Content::new("home", &post, &author);
        renderer.render_template(&publication, content);

        // let html_cid = crate::file_add(&html, &String::from("http://localhost"));
        // println!("{:?}", html_cid);
    
    }
}

