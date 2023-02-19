use lazy_static::lazy_static;
use regex::Regex;
use super::block::ParsingBlock;
use super::block::ParsingBlockCategory;

/// Returns a vector of ParsingBlock with all template blocks finded
///
/// # Arguments
///
/// * `text` - The original string with template blocks inside
pub fn extract_template_blocks(text: &str) -> Vec<ParsingBlock> {
    lazy_static! {
        static ref BLOCK_REGEX : Regex = Regex::new(
                r"(\{\{)[a-zA-Z0-9\(\) ]*(\}\})?"
            ).unwrap();
    }

    let mut result_vector: Vec<ParsingBlock> = Vec::new();
    for capture in BLOCK_REGEX.find_iter(text) {
        let mut capture_category = ParsingBlockCategory::Basic;
        let substring = &text[capture.start()..capture.end()];

        if substring.starts_with("{{include"){
            capture_category = ParsingBlockCategory::Include;
        } else if substring.starts_with("{{if"){
            capture_category = ParsingBlockCategory::Condition;
        } else if substring.starts_with("{{else"){
            capture_category = ParsingBlockCategory::ConditionElse;
        } else if substring.starts_with("{{endif"){
            capture_category = ParsingBlockCategory::ConditionEnd;
        } else if substring.starts_with("{{loop"){
            capture_category = ParsingBlockCategory::Iteration;
        } else if substring.starts_with("{{endloop"){
            capture_category = ParsingBlockCategory::IterationEnd;
        }

        let capture_block = ParsingBlock{
            
            category: capture_category,
            start: capture.start(),
            end: capture.end()
        };

        result_vector.push(capture_block);
        // check the captures of each match
        println!("{:?}", capture);
    }
    
    result_vector
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_template_blocks() {
        let template = "Test {{ block }} {{include test.txt}}. Test {{if(condition)}} ok1 {{else}} ok2 {{endif}}. IterTest {{loop(condition)}} ntimes {{endloop}}";
        let result = extract_template_blocks(template);
        assert_eq!(5,result[0].start);
        assert_eq!(ParsingBlockCategory::Include, result[1].category);
        assert_eq!(ParsingBlockCategory::Condition, result[2].category);
        println!("{}", &template[result[2].start..result[2].end]);
        assert_eq!(ParsingBlockCategory::ConditionElse, result[3].category);
        assert_eq!(ParsingBlockCategory::ConditionEnd, result[4].category);
        assert_eq!(ParsingBlockCategory::Iteration, result[5].category);
        assert_eq!(ParsingBlockCategory::IterationEnd, result[6].category);
    }
}


