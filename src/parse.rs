/* 
    This file has functions that help check if a string only contains the * character.
    It uses a simple step-by-step method to look at each letter in the string.
*/ 

// This function is not used right now.
pub fn parse(string: &str) -> Result<DataStructure, Error>
{
    println!("Parse function loaded");    
}

// Checks if the whole string is made up of only * characters.
pub fn string(string: &str) -> bool
{
    let mut state = State
    {
        cursor: 0,
        characters: string
            .chars()
            .collect::<Vec<_>>()
    };

    loop
    {
        if !unit(&mut state)
        {
            break
        }
    }

    state
        .cursor > 0 && state
            .cursor == state
                .characters
                    .len()
}

// Looks at one character and checks if it is a *
fn unit(state: &mut State) -> bool
{
    match state
        .characters
            .get(state
                .cursor)
    {
        Some(character) => if character == &'*'
        {
            state
                .cursor = state
                    .cursor + 1;

            true
        }
        else
        {
            false
        },
        None => false
    }
}

// Keeps track of where we are in the string and what the letters are
struct State
{
    cursor: usize,
    characters: Vec<char>
}
