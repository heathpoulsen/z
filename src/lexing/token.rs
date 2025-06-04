/*
    Writing Regex for Tokens

    Based on whatever we learned now we can understand why we wrote the regex of each token the way we did. Let's see:

    Keywords and Operators
        r"print", r"if", r"else", r"int\s+": These are straightforward. They match the specific keyword strings. For int, \s+ is added to ensure there's at least one whitespace character after the keyword, differentiating it from identifiers that might start with "int" (like integerValue).

    Literals

        r"\d+": This pattern matches integer literals. \d represents any digit, and + means one or more of the preceding elements, so this matches a sequence of digits.

        r#"\".*\"": This pattern matches string literals. The .* matches any character (except for line terminators) any number of times, and \" matches quotation marks.

    Identifiers
        r"[a-zA-Z_][a-zA-Z0-9_]* =": This matches identifiers. Identifiers start with a letter or underscore, followed by any number of letters, digits, or underscores. The space and = at the end ensure we're matching an assignment operation.

    Operators and Punctuation
        r"\+", r"=", r";", r"\(", r"\)", r"\{", r"}", r">", r"<": These patterns match operators and punctuation characters. In regex, some characters like +, (, ), {, } are special characters. To match them literally, they are escaped with a backslash \.

*/

// Allows us to print enum elements with println!() Without any extra effort

#[derive(Debug)]

pub enum Token
{
    // Keywords
    Print(String),
    If(String),
    Else(String),
    Int(String),

    // Literals
    IntegerLiteral(i32),
    StringLiteral(String),

    // Operators
    Plus(String),
    Assign(String),

    // Punctuation
    Semicolon(String),
    LeftParen(String),
    RightParen(String),
    LeftBrace(String),
    RightBrace(String),

    // Logical Operators
    GreaterThan(String),
    LessThan(String),

    // Identifiers
    Identifier(String),
}

// TRAITS TIME :D
impl Token
{
    // Gets the token type & value, if possible
    pub fn get_token(token_type: &str, value: Option<&str>)-> Token
    {
        // Looks through all possible decisions, & then does something based on the token type
        match token_type
        {
            "Print" => Token::Print("print"
                .to_string()),
            
            "If" => Token::If("if"
                .to_string()),

            "Else" => Token::Else("else"
                .to_string()),
            
            "Int" => Token::Int("int"
                .to_string()),

            "IntegerLiteral" => Token::IntegerLiteral(
                value
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
            ),

            "StringLiteral" => Token::StringLiteral(
                value
                    .unwrap()
                    .to_string()
            ),

            "Identifier" => Token::Identifier(
                value
                    .unwrap()
                    .to_string()
            ),

            "Plus" => Token::Plus("+".to_string()),

            "Assign" => Token::Assign("=".to_string()),

            "Semicolon" => Token::Semicolon(";".to_string()),

            "LeftParen" => Token::LeftParen("(".to_string()),

            "RightParen" => Token::RightParen(")".to_string()),

            "LeftBrace" => Token::LeftBrace("{".to_string()),

            "RightBrace" => Token::RightBrace("}".to_string()),

            "GreaterThan" => Token::GreaterThan(">".to_string()),

            "LessThan" => Token::LessThan("<".to_string()),

            // Fail-safe just in case none of the above matched
            _ => panic!("Unknown token type: {}", token_type),
        }
    }
}
