mod regex;
mod block;

use std::error::Error;
use std::collections::HashMap;

/// Parse template blocks in a string
/// 
/// # Arguments
/// 
/// * `content` - a string to parse
/// 
/// # Errors
/// If the string can't be parsed, an error will be returned.
/// 
pub fn parser(content: &String, data: &HashMap<String, String>) -> Result<String, Box<dyn Error>>{
   
    // Search for basic template
    let template_blocks = regex::extract_template_blocks(&content);
    
    // TODO Do all the job here
    println!("{:?}", template_blocks); // TODO remove
    println!("{:?}", data); // TODO remove

    Ok(String::from(""))
}