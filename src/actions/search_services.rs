// search and remove the lines that contain the query
pub fn search_and_remove<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    let query = query.to_lowercase();
    let exception = r#"r#"//""#;
    let exception2 = r#""//""#;
    let exception3 = r#""// ""#;

    contents.lines()
    .filter(|line|line.contains(exception)|| line.contains(exception2)  || line.contains(exception3) || !line.contains(&query) )
    .collect()
}

//search and comment out the lines that contain the query
pub fn search_and_comment(query: &str, contents: &str)->Vec<String>{
    let query = query.to_lowercase();
    
    contents.lines()
    .map(|line|{
        if line.contains(&query){
            if line.contains("//"){
                // run if line is already commented out
                line.to_string()
            }else{
                let space_count = line.chars()
                    .take_while(|&c| c.is_whitespace())
                    .count();

                " ".repeat(space_count)+ "// " + (line.trim_start())
            }
        }else{
            line.to_string()
        }
    })
    .collect()

}


pub fn search_and_un_comment(query: &str, contents: &str)->Vec<String>{
    let query = query.to_lowercase();
    
    contents.lines()
    .map(|line|{
        if line.contains(&query){
            if line.contains("//"){
                let space_count = line.chars()
                    .take_while(|&c| c.is_whitespace())
                    .count();
                let new_line = line.trim_start().strip_prefix("// ").expect("could not remove comment marks");
                " ".repeat(space_count).to_owned() + (new_line)
            }else{
                line.to_string()
            }
        }else{
            line.to_string()
        }
    })
    .collect()

}