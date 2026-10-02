use std::{fs};

pub fn get_contents(file_path:std::path::PathBuf )->String{
    let contents = fs::read_to_string(file_path).map_err(|er| {
        println!("error from reading file : {:#}", er.kind());
    });

    match contents {
        Ok(contant) => contant,
        Err(e) => format!("error from getting contants {:?}", e),
    }

}