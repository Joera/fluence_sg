use handlebars::Handlebars;
use handlebars::handlebars_helper;
use chrono::NaiveDate;
use serde_json::Value;
use std::collections::BTreeMap;
use serde::Serialize;


#[derive(Debug,Serialize)]
pub struct TemplateData  {
    pub body: BTreeMap<String, Value>,
    pub collections: BTreeMap<String, Vec<BTreeMap<String, Value>>>,
    pub base_url: String,
    pub assets_url: String,
    pub render_env: String
}

pub struct Renderer<'a> {
    handlebars: Handlebars<'static>,
    sqlite: &'a crate::sqlite::Sqlite
}

impl Renderer<'_> {

    pub fn new(sqlite: &crate::sqlite::Sqlite) -> Renderer {

        let mut handlebars = Handlebars::new();

        Renderer {
            handlebars,
            sqlite
        }
    }

    pub fn init(self: &mut Self) {

        handlebars_helper!(
            format_date: |date: Value| 
                NaiveDate::parse_from_str(&date.to_string().replace("\"",""), "%Y-%m-%d %H:%M:%S").unwrap().format("%B %Y").to_string()
            );
        self.handlebars.register_helper("format_date", Box::new(format_date));
        self.handlebars.register_helper("slugify", Box::new(crate::helpers::hbs_slugify));
        self.handlebars.register_helper("json", Box::new(crate::helpers::json));
        self.handlebars.register_helper("assign", Box::new(crate::helpers::assign_fct));
        self.handlebars.register_helper("if_equals", Box::new(crate::helpers::if_equals));
        self.handlebars.register_helper("map_unique", Box::new(crate::helpers::map_unique));
        self.handlebars.register_helper("filter", Box::new(crate::helpers::filter));

        for n in vec!("archive","list").iter() {
        // PARTIALS 
            let res = crate::read(&format!("partials/{}.handlebars", n),"/templates/");

            for e in res.errors.iter() {
                println!("{:?}", e);
            }
            if res.output.len() == 1 {
                self.handlebars.register_template_string(n, std::str::from_utf8(&res.output[0]).unwrap());
            }
        }
    }

    pub fn render_template(self: &mut Self, publication: &crate::publication::Publication, content: crate::content::Content)  {
        
        // println!("{:?}",content);
        let mut template: crate::publication::Template = publication.templates.iter().find( |t| &t.reference == content.post_type.as_ref().unwrap()).unwrap().clone();
        let res = crate::read(&template.file,"/templates/");

        // in template include 'path' & collections 
        // reason to delay rendering after content storage! 

        for e in res.errors.iter() {
            println!("{:?}", e);
        }
        if res.output.len() == 1 {
            
            // move to init ??
            self.handlebars.register_template_string("t1",  String::from_utf8(res.output[0].clone()).unwrap());
         
            let mut data = content.mapped.unwrap();

            let mut collections: BTreeMap<String,Vec<BTreeMap<String, Value>>> = BTreeMap::new();

            for c in template.collections.iter() {

                    let items : Vec<BTreeMap<String, Value>> = self.sqlite.query(&c.query);

                    collections.insert(
                        c.key.clone(),
                        items
                    );
            }


            let template_data = TemplateData {
                body: data.clone(),
                collections,
                base_url: "https://x.yz".to_string(),
                assets_url:"../assets".to_string(),
                render_env: "some_publisher".to_string()
            };
        
            let html = self.handlebars.render("t1", &template_data).unwrap();
            // do that thing with path from post_type 
            let file_name = template.path.replace("{slug}", &crate::helpers::slugify(data["title"].as_str().unwrap())).to_owned() ;
            // &format!("{}.html", 
            let folder = "/html/";
            
            crate::write(
                &file_name,
                folder,
                html.clone()
            );
                //      //   println!("{}{}", folder, file_name);
                //         // kan niet want folder en binary kennen elkaar niet zien
                //         // dus toch iets met @file
                //         // let html_cid = crate::file_add(&html, &String::from("http://localhost"));
                //         // println!("{:?}", html_cid);
            
        }
    }
}


// pub fn init_handlebars() -> Handlebars<'static> {

//     let mut handlebars = Handlebars::new();

//     // HELPERS

//     handlebars_helper!(
//         format_date: |date: Value| 
//             NaiveDate::parse_from_str(&date.to_string().replace("\"",""), "%Y-%m-%d %H:%M:%S").unwrap().format("%B %Y").to_string()
//         );
//     handlebars.register_helper("format_date", Box::new(format_date));
//     handlebars.register_helper("slugify", Box::new(crate::helpers::HbsSlugify));
    
//     for n in vec!("archive").iter() {
//     // PARTIALS 
//         let res = crate::read(&format!("partials/{}.handlebars", n),"/templates/");

//         for e in res.errors.iter() {
//             println!("{:?}", e);
//         }
//         if res.output.len() == 1 {
            
//             handlebars.register_template_string(n, std::str::from_utf8(&res.output[0]).unwrap());
//         }
//     }

//     handlebars

// }


// pub fn renderTemplate(handlebars: &mut Handlebars, publication: &crate::publication::Publication, content: crate::content::Content)  {

//    // println!("{:?}",content);
//     let mut template: crate::publication::Template = publication.templates.iter().find( |t| &t.reference == content.post_type.as_ref().unwrap()).unwrap().clone();
//     let res = crate::read(&template.file,"/templates/");

//     // in template include 'path' & collections 
//     // reason to delay rendering after content storage! 

//     for e in res.errors.iter() {
//         println!("{:?}", e);
//     }
//     if res.output.len() == 1 {
        
//         handlebars.register_template_string("t1",  String::from_utf8(res.output[0].clone()).unwrap());

//         let mut data = content.mapped.unwrap();

//         let mut collections = BTreeMap<String,String>::new();


//         for c in template.collections.iter() {

//                 let res = sqlite.run(c.query);

//                 collections.insert(
//                     c.key,

//                 )
//         }


//         let template_data = crate::types::TemplateObject {
//             body: data.clone(),
//             collections: vec!(data.clone()),
//             base_url: "https://x.yz".to_string(),
//             assets_url:"../assets".to_string(),
//             render_env: "some_publisher".to_string()
//         };
       
//         let html = handlebars.render("t1", &template_data).unwrap();
//         // do that thing with path from post_type 
//         let file_name = template.path.replace("{slug}", &crate::helpers::slugify(data["title"].as_str().unwrap())).to_owned() ;
//         // &format!("{}.html", 
//         let folder = "/html/";
        
//         crate::write(
//             &file_name,
//             folder,
//             html.clone()
//         );
//      //   println!("{}{}", folder, file_name);
//         // kan niet want folder en binary kennen elkaar niet zien
//         // dus toch iets met @file
//         // let html_cid = crate::file_add(&html, &String::from("http://localhost"));
//         // println!("{:?}", html_cid);
//     }
// }