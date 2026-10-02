use std::{fs, io};
use crate::models::Action;
use crate::actions::read_directory::read_dir;
use crate::actions::get_file_content::get_contents;
use crate::actions::search_services::{search_and_comment, search_and_remove,search_and_un_comment};


// Generic argument can take both String and &str arguments because they all implement as_ref
fn make_text<T: AsRef<str>>(results: Vec<T>)->String{
    let mut  override_text =String::new();

    for line in results {
        override_text.push_str(line.as_ref());
        override_text.push('\n');
    };

    override_text

}

pub fn run_action(query: &str, action: Action){
    let directory = read_dir("./");

    if let Ok(file) = directory{
        for path in file{

            let path_str = format!("{:#?}", path);

            if path_str.len() > 2{
                match action {
                    Action::CommentOut=>{
                        comment_and_override_existing_file(path, query);
                    }
                    Action::Remove=>{
                        delete_and_override_existing_file(path, query);
                    }
                    Action::UnComment=>{
                        remove_comment_from_println_and_override_existing_file(path, query);
                    }
                    Action::None=>{

                    }
                }
            }

        }
        
    }
}


// pub fn un_comment_printlns(query: &str){
//     let directory = read_dir("./");

//     if let Ok(file) = directory{
//         for path in file{

//             let path_str = format!("{:#?}", path);

//             if path_str.len() > 2{
//                 remove_comment_from_println_and_override_existing_file(path, query);
//             }

//         }
        
//     }
// }

// pub fn comment_printlns(query: &str){
//     let directory = read_dir("./");

//     if let Ok(file) = directory{
//         for path in file{

//             let path_str = format!("{:#?}", path);

//             if path_str.len() > 2{
//                 comment_and_override_existing_file(path, query);
//             }

//         }
        
//     }
// }

// pub fn remove_item(query: &str, ){
//     let directory = read_dir("./");

//     if let Ok(file) = directory{
//         for path in file{

//             let path_str = format!("{:#?}", path);

//             if path_str.len() > 2{
//                 delete_and_override_existing_file(path, query);
//             }

//         }
        
//     }
// }

pub fn remove_comment_from_println_and_override_existing_file(path: std::path::PathBuf, query: &str){
    let file_con  = get_contents(path.clone());  
    let results: Vec<String> = search_and_un_comment(query, &file_con);
    let _ = override_existing_file(path.clone(), &make_text(results));
}


pub fn comment_and_override_existing_file(path: std::path::PathBuf, query: &str){
    let file_con  = get_contents(path.clone());  
    let results: Vec<String> = search_and_comment(query, &file_con);
    let _ = override_existing_file(path.clone(), &make_text(results));
}

pub fn delete_and_override_existing_file(path: std::path::PathBuf, query: &str){
    let file_con  = get_contents(path.clone());  
    let results: Vec<&str> = search_and_remove(query, &file_con);
    let _ = override_existing_file(path.clone(), &make_text(results));
}



fn override_existing_file(path: std::path::PathBuf, override_text: &str)->io::Result<()>{
    fs::write(path, override_text)?;

    Ok(())
}
