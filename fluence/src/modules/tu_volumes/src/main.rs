#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use tu_types::results::{ AquaMarineResultVecU8, AquaMarineResultString };

mod filesystem;

pub fn main() {}

#[marine]
pub fn read(file_name: &str, path: &str) -> AquaMarineResultVecU8 {

    let call_data = marine_rs_sdk::get_call_parameters();
    let mut output: Vec<Vec<u8>> = vec!();
    let mut errors: Vec<String> = vec!();

    
    if let Ok(_result) =  filesystem::read(path, &file_name) {
        output.push(_result);
    } else { 
        errors.push(format!("could not read file {} from folder {} on {}", file_name, path, call_data.host_id))
    }

    AquaMarineResultVecU8 {
        output,
        errors
    }
}

#[marine]
pub fn write(file_name: &str, path: &str, data: String) -> AquaMarineResultString {

    let call_data = marine_rs_sdk::get_call_parameters();
    let mut output: Vec<String> = vec!();
    let mut errors: Vec<String> = vec!();

    
    if let Ok(_result) =  filesystem::write(file_name, path, data) {
        output.push(format!("{}{}", path,file_name));
    } else { 
        errors.push(format!("could not write file {} to folder {} on {}", file_name, path, call_data.host_id))
    }

    AquaMarineResultString {
        output,
        errors
    }
}

