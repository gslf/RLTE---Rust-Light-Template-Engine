/// Categories of available parsing block
enum ParsingBlockCategory{
    Basic,
    Include,
    Condition,
    Iteration,
}

// Parsing Block type definition
struct ParsingBlock{
    pub category: ParsingBlockCategory,
    pub position: isize
}