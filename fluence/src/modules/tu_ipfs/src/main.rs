#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use tu_types::results::{ AquaMarineResultVecU8, AquaMarineResultString };
use serde_json::Value;

module_manifest!();

fn main() {

}

#[marine]
pub fn file_add(data: &String, remote_ipfs: &String) -> AquaMarineResultString {

    let call_data = marine_rs_sdk::get_call_parameters();
    let mut output: Vec<String> = vec!();
    let mut errors: Vec<String> = vec!();

    let url = format!("{}:5001/api/v0/add", remote_ipfs); 
    let data_string = format!("file={:?}", data);

    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("-F"),
        data_string,
        url
    ];

    let response = curl(curl_args);
    
    let hash = extract_hash(&String::from_utf8(response.stdout).unwrap());
    output.push(hash);

    AquaMarineResultString {
        output,
        errors
    }
}

#[marine]
pub fn ipfs_hash(data: &String, remote_ipfs: &String) -> AquaMarineResultString {

    let call_data = marine_rs_sdk::get_call_parameters();
    let mut output: Vec<String> = vec!();
    let mut errors: Vec<String> = vec!();

    let url = format!("{}:5001/api/v0/add?only-hash=true", remote_ipfs); 
    let data_string = format!("file={:?}", data);

    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("-F"),
        data_string,
        url
    ];

    let response = curl(curl_args);

   //  println!("{:?}", response);
    // println!("{:?}", response.stderr);

    let hash = extract_hash(&String::from_utf8(response.stdout).unwrap());
    output.push(hash);

    AquaMarineResultString {
        output,
        errors
    }
}

#[marine]
pub fn dag_put(obj: String, codec: &str, remote_ipfs: &String) -> AquaMarineResultString {

    let call_data = marine_rs_sdk::get_call_parameters();
    let mut output: Vec<String> = vec!();
    let mut errors: Vec<String> = vec!();

    let v = serde_json::to_value(obj).unwrap();

    let url = format!("{}:5001/api/v0/dag/put?store-codec={}&pin=true", remote_ipfs,codec.to_string());

    let data_string = format!("file={}", v);
  
    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("-F"),
        data_string,
        url
    ];
  
    let response = curl(curl_args);

    // if let Ok(_result) =  filesystem::read(path, &file_name) {
    output.push(String::from_utf8(response.stdout).unwrap());
    // } else { 
    //     errors.push(format!("could not read file {} from folder {} on {}", file_name, path, call_data.host_id))
    // }

    AquaMarineResultString {
        output,
        errors
    }
}

pub fn extract_hash (response: &String) -> String {
    let v : serde_json::Value = serde_json::from_str(response).unwrap();
    v["Hash"].as_str().unwrap().to_string()
}


#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}

