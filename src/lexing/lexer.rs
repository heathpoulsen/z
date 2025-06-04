// Importing the libaries
use regex::Regex;

// If this becomes Super, then it will cause an error
use crate::lexing::token::
{
    Token
};

pub fn lexer(program: &str) -> Vec<Token>
{
    let current_input = program;

    let tokens = [

        // Oh no..
        "Print",
        "If",
        "Else",
        "Int",
        "Plus",
        "Assign",
        "Semicolon",
        "LeftParen",
        "RightParen",
        "LeftBrace",
        "RightBrace",
        "GreaterThan",
        "LessThan",
        "IntegerLiteral",
        "StringLiteral",
        "Identifier", // The lowest priority thing (Sucks to be this guy fr)
    ];

    let mut match_vec: Vec<(&str, usize, usize)> = Vec::new();

    // Iterate through each token in the array of tokens
    for token in tokens
        .iter()
    {
        // Get the regex for the token
        let token_regex = Token::get_token_regex(token);

        // Initalize the regex object with the token regex string
        let re = Regex::new(token_regex.as_str()).unwrap();

        // Find all of the current matches in re
        let matched = re.find_iter(current_input);

        // Gets all of the matches into a vector (list)
        let all_matches = matched
            .collect::<Vec<_>>();

        // Check if there are any matches
        if all_matches
            .len() == 0
        {
            continue;
        }
        
        // Store the match details
        for m in all_matches
        {
            match_vec
                .push((token, m
                    .start(), m
                        .end()));
        }
    }

    // Sorting the vector
    match_vec
        .sort_by(|a, b | a
            .1
            .cmp(&b
                .1)
                    .then_with(||(b
                        .2 - b
                            .1)
                            .cmp(&(a
                                .2 - a
                                    .1))));

    // Iterate over the match_vectors
    let mut token_vec: Vec<Token> = Vec::new();

    for m in match_vec
        .iter()
    {
        token_vec
            .push(Token::get_token(m
                .0, Some(&current_input[m
                    .1 ..m
                        .2])));
    }

    // Returns the token vector
    token_vec
}
// If it ain't broke, then don't fix it
