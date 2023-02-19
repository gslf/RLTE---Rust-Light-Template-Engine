/// Categories of available parsing block
#[derive(Debug, PartialEq)]
pub enum ParsingBlockCategory{
    Basic,
    Include,
    Condition,
    ConditionElse,
    ConditionEnd,
    Iteration,
    IterationEnd,
}

// Parsing Block type definition
#[derive(Debug, PartialEq)]
pub struct ParsingBlock{
    pub category: ParsingBlockCategory,
    pub start: usize,
    pub end: usize
}