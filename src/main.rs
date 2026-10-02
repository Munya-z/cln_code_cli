use std::env;

mod models;
mod handlers;
mod actions;
use crate::models::{Config};
use crate::handlers::run;

fn main (){
    let env_args : Vec<String> = env::args().collect();

    let args= Config::new(&env_args).map_err(|err|{
        eprintln!("problem parsing arguments {}", err);
    }).unwrap();

    run(args);

    println!("proccess completed seccessfully")
}






