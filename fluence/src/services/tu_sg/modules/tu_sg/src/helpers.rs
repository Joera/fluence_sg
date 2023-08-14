use regex::Regex;
use std::io::Write;
use handlebars::{
    Handlebars, 
    HelperDef, 
    RenderContext, 
    Helper, 
    Context, 
    JsonRender, 
    HelperResult, 
    Output, 
    RenderError, 
    PathAndJson, 
    ScopedJson
};
use serde_json::{json,Value};
use thiserror::Error;

#[derive(Debug, Error)]
enum HelperError {
    #[error("missing param {position} '{name}' of '{helper_signature}'")]
    MissingParameter {
        position: usize,
        name: String,
        helper_signature: String,
    },
}


pub fn slugify(input: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9_-]").unwrap();

    let slug = re.replace_all(input, "-");
    slug.to_lowercase()
}



pub fn hbs_slugify (h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let input = h.param(0).unwrap().value().as_str().unwrap();
    let slug = slugify(input);
    out.write(slug.as_ref())?;
    Ok(())
}

pub fn if_equals (h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let v1 = h.param(0).unwrap().value().as_str().unwrap();
    let v2 = h.param(1).unwrap().value().as_str().unwrap();

    if v1 != v2 {
        h.inverse();
    }
   
    Ok(())
    
}

// map .. unique 

pub fn map_unique(
    h: &Helper, 
    _: &Handlebars, 
    ctx: &Context, 
    rc: &mut RenderContext, 
    out: &mut dyn Output
) -> HelperResult {

    let key = h.param(0).and_then(|v| v.value().as_str()).ok_or_else(|| {
        RenderError::from_error(
            "missing parameter",
            HelperError::MissingParameter {
                position: 0,
                name: "var_name".to_owned(),
                helper_signature: "assign var_name value".to_owned(),
            },
        )
    })?;

    let input = h.param(1).map(|v| v.value()).cloned().ok_or_else(|| {
        RenderError::from_error(
            "missing parameter",
            HelperError::MissingParameter {
                position: 1,
                name: "value".to_owned(),
                helper_signature: "assign var_name value".to_owned(),
            },
        )
    })?;

    if let Some(posts) = input.as_array() {
        
        let years: Vec<Value> = posts
            .iter()
            .map(|p| {
                p["year"].clone()
            })
            .collect();

        let mut unique_years: Vec<Value> = vec!();

        for year in &years {
            if !unique_years.contains(&year) {
                unique_years.push(year.clone());
            }
        }
   
        let mut ctx = ctx.clone();
        match ctx.data_mut() {
            serde_json::value::Value::Object(m) => m.insert(key.to_owned(), serde_json::Value::Array(unique_years)),
            _ => None,
        };
        rc.set_context(ctx);
   
    }

    Ok(())
}

pub fn filter(
    h: &Helper, 
    _: &Handlebars, 
    ctx: &Context, 
    rc: &mut RenderContext, 
    out: &mut dyn Output
) -> HelperResult {

    let key = h.param(0).and_then(|v| v.value().as_str()).ok_or_else(|| {
        RenderError::from_error(
            "missing parameter",
            HelperError::MissingParameter {
                position: 0,
                name: "var_name".to_owned(),
                helper_signature: "assign var_name value".to_owned(),
            },
        )
    })?;

    let value = h.param(1).and_then(|v| v.value().as_str()).ok_or_else(|| {
        RenderError::from_error(
            "missing parameter",
            HelperError::MissingParameter {
                position: 0,
                name: "var_name".to_owned(),
                helper_signature: "assign var_name value".to_owned(),
            },
        )
    })?;

    let input = h.param(2).map(|v| v.value()).cloned().ok_or_else(|| {
        RenderError::from_error(
            "missing parameter",
            HelperError::MissingParameter {
                position: 1,
                name: "value".to_owned(),
                helper_signature: "assign var_name value".to_owned(),
            },
        )
    })?;

    if let Some(posts) = input.as_array() {
 
        let filtered_posts: Vec<Value> = posts
            .iter()
            .filter(|&p| p[key] == value)
            .cloned()
            .collect();

        let mut ctx = ctx.clone();
        
        match ctx.data_mut() {
            serde_json::value::Value::Object(m) => m.insert("filter".to_owned(), serde_json::Value::Array(filtered_posts)),
            _ => None,
        };
        rc.set_context(ctx);
    }
    
    Ok(())
}


pub fn json (
    h: &Helper, 
    _: &Handlebars, 
    _: &Context, 
    rc: &mut RenderContext, 
    out: &mut dyn Output
) -> HelperResult {

    let v = h.param(0);

    if v.is_some() {
        let input = serde_json::to_string(h.param(0).unwrap().value()).unwrap(); // .as_str().unwrap();
        out.write(input.as_ref())?;
    }
    
    Ok(())
}

pub fn assign_fct(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    rc: &mut RenderContext,
    _: &mut dyn Output,
) -> HelperResult {


    // get parameter from helper or throw an error
    let name = h.param(0).and_then(|v| v.value().as_str()).ok_or_else(|| {
        RenderError::from_error(
            "missing parameter",
            HelperError::MissingParameter {
                position: 0,
                name: "var_name".to_owned(),
                helper_signature: "assign var_name value".to_owned(),
            },
        )
    })?;
    let value = h.param(1).map(|v| v.value()).cloned().ok_or_else(|| {
        RenderError::from_error(
            "missing parameter",
            HelperError::MissingParameter {
                position: 1,
                name: "value".to_owned(),
                helper_signature: "assign var_name value".to_owned(),
            },
        )
    })?;
    let mut ctx = ctx.clone();
    match ctx.data_mut() {
        serde_json::value::Value::Object(m) => m.insert(name.to_owned(), value),
        _ => None,
    };
    rc.set_context(ctx);

    Ok(())
}
