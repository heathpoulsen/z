// use lexing::lexer;

/*
    a = 10 if 10<12 else 12

    [IDENTIFIER, 'a']  [ASSIGN_OP, '=']  [INTEGER, '10']  [IF]  [INTEGER, '10']  [LESS_THAN]  [INTEGER, '12']  [ELSE]  [INTEGER, '12']

    This is an example of a lexer, a lexer knows about white-spaces & comments, but it just chooses to ignore them.


    Regex String Rules:

    Start and End Anchors: ^ and $ are used to denote the beginning and end of a line, respectively. ^ matches the position before the first character in the string, and $ matches the position after the last character in the string.

    Character Classes: [ ] are used to match any one of the characters enclosed within the brackets. For example, [abc] will match any single 'a', 'b', or 'c'. Ranges can also be specified, like [a-z] for any lowercase letter and [0-9] for any digit.

    Predefined Character Classes: These are shortcuts for common character classes. For instance, \d matches any digit (equivalent to [0-9]), \w matches any word character (equivalent to [a-zA-Z0-9_]), and \s matches any whitespace character (like space, tab, newline).

    Negated Character Classes: Placing a ^ inside square brackets negates the class. For example, [^abc] matches any character except 'a', 'b', or 'c'.

    Quantifiers: These specify how many instances of a character or group are needed for a match. * matches zero or more, + matches one or more, ? matches zero or one, and {n}, {n,}, {n,m} are used for specific quantities.

    This syntax is usually the same for regex across any language, so this will apply to Rust, Python, or any other language. But theory can only take you so far, so let's start implementing the lexer and building regex for every keyword, identifier, operation etc.
*/

mod lexing;
mod lexer;
mod token;

// Importing the libaries

const PROGRAM: &str = "
int x = 5;
int y = 6;
int z = x + y;

if (z > 10) {
    print(\"Hello, world!\");
} else {
    print(\"Goodbye, world!\");
}
";

// Dummy implementation for demonstration.
// Replace with your actual lexing logic.
fn lex_program(_input: &str) -> Vec<String> {
    vec!["TOKEN1".to_string(), "TOKEN2".to_string()]
}

// Dummy implementation for demonstration.
// Replace with your actual string logic.
fn string(input: &str) -> &str {
    input
}

fn main()
{
    let tokens = lex_program(PROGRAM);

    for token in tokens
        .iter()
    {
        println!("{:?}", token);
    }

    let test_string = "****";
    println!("Result: {}", string(test_string));

    lexer::do_lexing("Yes"); // do_lexing is an error message
}
