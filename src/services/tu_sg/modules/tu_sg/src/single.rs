#![allow(unused)]

use std::collections::HashMap;
use serde_json::{Value, from_slice, from_value, json};
use crate::BTreeMap;

use handlebars::Handlebars;
use marine_sqlite_connector;
use marine_sqlite_connector::{Connection};



pub fn render (author: String, publisher: String, content: String) {

    // get content_owner config

    let sqlite: Connection = marine_sqlite_connector::open(":memory:").unwrap();

    crate::content::create_table(&sqlite);

    let mut field_mappings = HashMap::new();
    field_mappings.insert(
        "title".to_owned(),
        vec!("title".to_owned())
    );
    field_mappings.insert(
        "content".to_owned(),
        vec!("content".to_owned())
    );
    field_mappings.insert(
        "date".to_owned(),
        vec!("date".to_owned())
    );

    field_mappings.insert(
        "thumbnail".to_owned(),
        vec!("attachments".to_owned(), "thumbnail".to_owned())
    );

    let mut content_mappings = HashMap::new();
    content_mappings.insert(
        "post".to_owned(),
        field_mappings
    );


    // mock for now: 
    let author = crate::types::Author {
        reference: "123".to_owned(),
        repository : "linktorepo".to_owned(),
        public_key: "123".to_owned(),
        content_mappings
    };

    // vecs are more efficient for smaller collections .. 
    let mut templates = vec!();

   // use hashmap or find in vec? 
    templates.push(
        crate::types::Template {
            reference: "post".to_string(),
            file: "post.handlebars".to_string()
        }
    );

    // get publisher config
    // mock for now: 
    let publisher = crate::types::Publisher {
        reference: "123".to_owned(),
        domain: "xxx.eth".to_owned(),
        location: "xxx".to_owned(),
        templates
    };

    // get content
    // for now: read from file ? 
    let res = crate::read("nieuwsarchief.json","/content/");

    for e in res.errors.iter() {
        println!("{:?}", e);
    }

    if res.output.len() == 1 {
        let raw: Value = from_slice(&res.output[0]).unwrap();

        if let Some(post_array) = raw["posts"].as_array() {

            for post in post_array {

                // pass through content mapping
                let mut content = crate::Content::new(&post, &author);
                
                // ?? store in ipld or sql-lite --> some form of both .. on ipfs .. but use sql-lite for quering 
                content.store(&sqlite);

                // find template from publisher 
                let mut template: crate::types::Template = publisher.templates.iter().find( |t| &t.reference == content.post_type.as_ref().unwrap()).unwrap().clone();
                // to do: create render queue // in sql-lite? 
                // then split into scheduler and renderer 
                template = template.queue();
                
                let mut data = content.mapped.unwrap(); // from_value::<BTreeMap<String, Value>>(content.mapped.unwrap()).unwrap();

                let template_data = crate::types::TemplateObject {
                    body: data.clone(),
                    collections: vec!(data.clone()),
                    base_url: "https://x.yz".to_string(),
                    assets_url:"https://x.yz".to_string(),
                    render_env: "some_publisher".to_string()
                };

                let mut handlebars = Handlebars::new();
                // register helpers
                // register partials
                // read template 
                let res = crate::read(&template.file,"/templates/");

                for e in res.errors.iter() {
                    println!("{:?}", e);
                }
                if res.output.len() == 1 {
                    handlebars.register_template_string("t1",  String::from_utf8(res.output[0].clone()).unwrap());
                    let html = handlebars.render("t1", &template_data).unwrap();
                    let file_name =  &format!("{}.html", crate::helpers::slugify(data["title"].as_str().unwrap())).to_owned();
                    let folder = "/html/";
                    crate::write(
                        file_name,
                        folder,
                        html.clone()
                    );
                    // kan niet want folder en binary kennen elkaar niet zien
                    // dus toch iets met @file
                    let html_cid = crate::file_add(&html, &String::from("http://localhost"));
                    println!("{:?}", html_cid);
                }

                

                // what is the content hash 
                // in sql lite store contenthash + templateData + ??? (author + publisher = deal?)
            }

        }

        // lets make a temp solution for rendering homepage

        let posts: Vec<BTreeMap<String, Value>> = crate::content::get_posts(&sqlite);

       // println!("{:?}", posts[0].mapped);

       let body: BTreeMap<String, Value> = BTreeMap::new();

        let template_data = crate::types::TemplateObject {
            body,
            collections: posts,
            base_url: "https://x.yz".to_string(),
            assets_url:"../assets/".to_string(),
            render_env: "some_publisher".to_string()
        };

        let mut handlebars = Handlebars::new();
        handlebars.register_helper("slugify", Box::new(crate::helpers::HbsSlugify));

        let res = crate::read("home.handlebars","/templates/");

        for e in res.errors.iter() {
            println!("{:?}", e);
        }
        if res.output.len() == 1 {
            handlebars.register_template_string("t1",  String::from_utf8(res.output[0].clone()).unwrap());
            let html = handlebars.render("t1", &template_data).unwrap();
            let folder = "/html/";
            crate::write(
                "index.html",
                folder,
                html.clone()
            );
            // kan niet want folder en binary kennen elkaar niet zien
            // dus toch iets met @file
            let html_cid = crate::file_add(&html, &String::from("http://localhost"));
            println!("{:?}", html_cid);
        }
    }

   



    // queue primnaries
    // queue ripples

    // w. with queue
    // render 
    // fs / ipfs 
    // 


    // modules
        // queue
        // template
        // ipfs 

}

