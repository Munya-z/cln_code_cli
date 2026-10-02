use std::{fs};
use std::result::Result;
use std::io;

pub fn read_dir(dir: &str)->Result<Vec<std::path::PathBuf>, io::Error>{

    println!("reading directory ...");
    let dir_contents = fs::read_dir(dir)?
        .map(|res| res.map(|e|{ 

            let metadata = fs::metadata(e.path()).expect("could not get metadata");

            let name = e.file_name().into_string().expect("could not convet name to string");
            
            let file_type = metadata.file_type();

            if !file_type.is_dir() && name.contains(&".rs".to_string())  || name.contains(&".txt".to_string()){
                e.path()
            }else {
                std::path::PathBuf::new()
            }

        }))
        .collect::<io::Result<Vec<std::path::PathBuf>>>()?;
    
    Ok(dir_contents)
}
