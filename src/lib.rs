/*!
 * #  RLTEn - Rust Light Template Engine
 * 
 * A rust library to render templates from file or from strings. 
 * 
 * This simple template engine it's a part of an articulated project for the generation of complex static content. 
 * rust The template syntax is simple and uses four ideomatic models to manage substitutions, conditional constructs and cyclic iterations.
 * 
 * ## Available patterns
 * - {{variable}} => Replaced with the content of the "variable" content in the model data HashMap
 * - {{include path/file.txt}} => Replaced with the contenet of file "path/file.txt"
 * - {{if(condition)}} {{else}} {{endif}} => Conditional replacing  (else block is optional)
 * - {{loop(condition)}} {{endloop}} => Iterative replacing
 * 
 * Written with phylosophy in southern Italy by **Gioele SL Fierro** 
 * 
 * [gslf.it](https://gslf.it)
 * 
 * ## Usage
 * 
 * ## Syntax Example
 *
*/

mod engine;

use std::collections::HashMap;
use std::fs;
use std::error::Error;

/// The representation of a model template
pub struct Model{
    /// The content of the model
    content: String,
    
    /// The data to insert in the template
    data: HashMap<String, String>
}

impl Model{

    /// Create a model loading a file from path.
    /// 
    /// # Arguments
    ///
    /// * `path` - A string with the file path of the model
    ///
    /// # Examples
    ///
    /// ```
    /// use rlte;
    /// use std::collections::HashMap;
    /// let data:HashMap<String, String> = HashMap::new();
    /// let model = rlte::Model::from_file(String::from("path/file.txt"), data);
    /// ```
    pub fn from_file(path: String, data: HashMap<String, String>) -> Result<Self, Box<dyn Error>>{
        
        let content_string = fs::read_to_string(path)?;
        println!("{}", content_string);
        Ok(Self { 
            content: content_string,
            data: data
        })
    }

    /// Create a model from string.
    /// 
    /// # Arguments
    ///
    /// * `content` - A string with the content of the model
    ///
    /// # Examples
    ///
    /// ```
    /// use rlte;
    /// use std::collections::HashMap;
    /// let content = String::from("A string that contain the <@ model @>");
    /// let data:HashMap<String, String> = HashMap::new();
    /// let model = rlte::Model::from_string(content, data);
    /// ```
    pub fn from_string(content: String, data: HashMap<String, String>) -> Self{
        Self { 
            content: content,
            data: data
        }
    }

    /// Get the reference of the content string.
    /// 
    ///
    /// # Examples
    ///
    /// ```
    /// use rlte;
    /// use std::collections::HashMap;
    /// let data:HashMap<String, String> = HashMap::new();
    /// let model = rlte::Model::from_string(String::from("A string that contain the <@ model @>"), data);
    /// println!("{:?}", model.content());
    /// ```
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Render the model and return a composed string using the template engine.
    /// 
    /// 
    /// # Errors
    /// If the model can't be rendered, an error will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rlte;
    /// use std::collections::HashMap;
    /// let data:HashMap<String, String> = HashMap::new();
    /// let model = rlte::Model::from_string(String::from("A string that contain the <@ model @>"), data);
    /// println!("{:?}", model.render());
    /// ```
    pub fn render(&self) -> Result<String, Box<dyn Error>>{
        let rendered = engine::parser(&self.content, &self.data) ?;
        Ok(rendered)
    }

    /// Render the model and write the rendered version to a file.
    /// 
    /// # Arguments
    /// 
    /// * `path` - A string with the destination file path
    /// 
    /// # Errors
    /// If the file can't be writed, an error will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rlte;
    /// use std::collections::HashMap;
    /// let data:HashMap<String, String> = HashMap::new();
    /// let model = rlte::Model::from_string(String::from("A string that contain the <@ model @>"), data);
    /// println!("{:?}", model.render());
    /// ```
    pub fn render_to_file(&self, path:String) -> Result<(),Box<dyn Error>> {
        let rendered = engine::parser(&self.content, &self.data) ?;
        fs::write(path, rendered) ?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let data:HashMap<String, String> = HashMap::new();
        let doc = Model::from_file(String::from("tests/test.txt"), data).unwrap();
        assert_eq!(doc.content(), "Test");
        assert!(doc.render().is_ok());
    }

    #[test]
    fn test_render_to_file(){
        let data:HashMap<String, String> = HashMap::new();
        let doc = Model::from_string(String::from("Test"), data);
        assert!(doc.render_to_file(String::from("tests/test2.txt")).is_ok());
    }

    // TODO: Write more tests
}