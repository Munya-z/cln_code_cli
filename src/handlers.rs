
use crate::models::{Action, Item,Location, Config};
use crate::actions::handlers::{delete_and_override_existing_file,comment_and_override_existing_file,run_action, remove_comment_from_println_and_override_existing_file};

pub fn run(args: Config){
    match args.location {
        Location::All =>{
            match args.action {
                Action::Remove=>{
                    remove(args);
                }
                Action::CommentOut =>{
                    comment_out(args);
                }
                Action::UnComment =>{
                    remove_comment(args);
                }
                Action::None =>{
                    panic!("fail to do work: no arguments match what is required")
                }
            }
        }
        Location::FilePath(value) =>{
            let path = std::path::PathBuf::from(value);
            match args.action {
                Action::Remove=>{
                    match args.item {
                        Item::Printlns=>{
                            let query = r#"println!"#;
                            delete_and_override_existing_file(path,query);
                        }
                        Item::Comments =>{
                            let query = r#"//"#;
                            delete_and_override_existing_file(path,query);
                        }
                        Item::None =>{
                            panic!("fail to do work: no arguments match what is required")
                        }
                    }
                }
                Action::CommentOut =>{
                    match args.item{
                        Item::Printlns=>{
                            let query = r#"println!"#;
                            comment_and_override_existing_file(path,query);
                        }
                        Item::Comments =>{
                            panic!("you can't comment out comments you AI HAHAHAHA")
                        }
                        Item::None =>{
                            panic!("fail to do work: no arguments match what is required")
                        }
                    }
                }
                Action::UnComment =>{
                    match args.item{
                        Item::Printlns=>{
                            let query = r#"println!"#;
                            remove_comment_from_println_and_override_existing_file(path,query);
                        }
                        Item::Comments =>{
                            panic!("removing comment from comments might break your code dummy ")
                        }
                        Item::None =>{
                            panic!("fail to do work: no arguments match what is required")
                        }
                    }
                }
                Action::None =>{
                    panic!("fail to do work: no arguments match what is required")
                }
            }
        }

        }
    }

fn remove(args: Config){
    match args.item {
        Item::Printlns=>{
            let query = r#"println!"#;
            run_action(query , args.action);
        }
        Item::Comments =>{
            let query = r#"//"#;
            run_action(query , args.action);
        }
        Item::None =>{
            panic!("fail to do work: no arguments match what is required")
        }
    }
}

fn comment_out(args: Config){
    match args.item{
        Item::Printlns=>{
            let query = r#"println!"#;
            run_action(query , args.action);
        }
        Item::Comments =>{
            panic!("you can't comment out comments you AI HAHAHAHA")
        }
        Item::None =>{
            panic!("fail to do work: no arguments match what is required")
        }
    }
}

fn remove_comment(args: Config){
    match args.item{
        Item::Printlns=>{
            let query = r#"println!"#;
            run_action(query , args.action);
        }
        Item::Comments =>{
            panic!("you can't comment out comments you AI HAHAHAHA")
        }
        Item::None =>{
            panic!("fail to do work: no arguments match what is required")
        }
    }
}

