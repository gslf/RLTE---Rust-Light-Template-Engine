use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

/// Returns an HashSet of strings with all template blocks finded
///
/// # Arguments
///
/// * `text` - The original string with template blocks inside
pub fn extract_template_blocks(text: &str) -> HashSet<&str> {
    // TODO: Change the template block to find ({{}}) and the return value (a vector of parsing block)
    lazy_static! {
        static ref HASHTAG_REGEX : Regex = Regex::new(
                r"(@<)[a-zA-Z0-9 ]*(>@)?"
            ).unwrap();
    }
    HASHTAG_REGEX.find_iter(text).map(|mat| mat.as_str()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_template_blocks() {
        let result = extract_template_blocks("Test @< block >@ .");
        assert_eq!(result.into_iter().collect::<Vec<&str>>().join(""), "@< block >@");
    }
}


