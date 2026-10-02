pub struct Config {
    pub action: Action,
    pub item: Item,
    pub location: Location,
}

pub enum Action{
    Remove,
    CommentOut,
    UnComment,
    None,
}

pub enum Location{
    All,
    FilePath(String),
}

pub enum Item{
    Printlns,
    Comments, 
    None
}

impl Item{
    pub fn new(item: String)->Item{
        if item == "com"{
            Item::Comments
        }
        else if item == "print" || item == "log"{
            Item::Printlns
        }
        else{
            Item::None
        }
    }
}

impl Action{
    pub fn new(item: String)->Action{
        if item.contains("rm"){
            Action::Remove
        }
        else if item == "cm" || item == "co" {
            Action::CommentOut
        }
        else if item == "uc" || item == "rc" {
            Action::UnComment
        }
        else{
            Action::None
        }
    }
}

impl Location{
    pub fn new(item: String)->Location{
        if item.contains("all"){
            Location::All
        }
        else {
            Location::FilePath(item)
        }
    }
}

impl Config{

    pub fn new(args: &[String])->Result<Self, &'static str>{
        if args.len() != 4{
            return Err(r#"No the right amount or arguments provided.
    App requres 3,
    1: action(rm : for remove or co/cm : for commenting  or uc/rc for uncommenting) 
    2: target(com : for comments or print/log : for println!)
    3: file(all : for the current directory or [your_file_name] : for specific file)"#);
        }
        let location = Location::new(args[3].clone());
        let action = Action::new(args[1].clone());
        let item = Item::new(args[2].clone());

        Ok(Self{
            action,
            item,
            location,
        })
    }
}
