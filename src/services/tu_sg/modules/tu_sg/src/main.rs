#![allow(
    unused
)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use std::collections::{ BTreeMap};
use tu_types::results::{AquaMarineResultString, AquaMarineResultVecU8};


module_manifest!();

mod types;
mod single;
mod bulk;
mod content;
mod template;
mod helpers;

use content::Content;


pub fn main() {}

#[marine]
pub fn render(author: String, publisher: String, content: String) -> AquaMarineResultString {

    let _call_data = marine_rs_sdk::get_call_parameters();
    let mut output: Vec<String> = vec!();
    let mut errors: Vec<String> = vec!();

    let report = single::render(author, publisher, content);

    AquaMarineResultString {
        output,
        errors
    }
}

// #[marine]
// pub fn bulkrender(author: String, publisher: String, page_type: String) -> AquaMarineResultString {
    
//     let _call_data = marine_rs_sdk::get_call_parameters();
//     let mut output: Vec<String> = vec!();
//     let mut errors: Vec<String> = vec!();

//     let report = bulk::render(author, publisher, page_type);

//     AquaMarineResultString {
//         output,
//         errors
//     }

// }

#[marine]
#[link(wasm_import_module = "tu_volumes")]
extern "C" {
    pub fn read(file_name: &str, path: &str ) -> AquaMarineResultVecU8;
    pub fn write(file_name: &str, path: &str, data: String ) -> AquaMarineResultString;
}

#[marine]
#[link(wasm_import_module = "tu_ipfs")]
extern "C" {
    pub fn ipfs_hash(data: &String, remote_ipfs: &String) -> AquaMarineResultString;
    pub fn file_add(data: &String, remote_ipfs: &String) -> AquaMarineResultString;
    pub fn dag_put(obj: String, codec: &str, remote_ipfs: &String) -> AquaMarineResultString;
}
