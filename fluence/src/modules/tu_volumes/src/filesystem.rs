#![allow(unused)]

use std::fs;
use std::path::PathBuf;
use std::io;

pub fn write(

    file_name: &str,
    path: &str, 
    file_content: String,

) -> Result<(), io::Error> {

    let tmp_filepath = format!("{}{}", path, file_name);
    fs::write(PathBuf::from(tmp_filepath.clone()), file_content)
}

pub fn read(
    
    path: &str, 
    file_name: &str

) -> Result<Vec<u8>, io::Error>  {

    let tmp_filepath = format!("{}{}", path, file_name);
    fs::read(tmp_filepath)
}
