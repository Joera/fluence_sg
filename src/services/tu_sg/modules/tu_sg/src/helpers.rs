use regex::Regex;
use std::io::Write;
use handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, JsonRender, HelperResult, Output, RenderError};


pub fn slugify(input: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9_-]").unwrap();

    let slug = re.replace_all(input, "-");
    slug.to_lowercase()
}

#[derive(Clone, Copy)]
pub struct HbsSlugify;

impl HelperDef for HbsSlugify {

    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
        
        let param = slugify(h.param(0).unwrap().value().as_str().unwrap());
        out.write(param.as_ref())?;
        Ok(())
      }
} 