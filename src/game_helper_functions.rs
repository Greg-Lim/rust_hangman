
use std::io::stdin;


#[derive(Debug)]
pub enum Guess {
    CharGuess(char),
    StringGuess(String),
}

pub fn get_input() -> Guess {
    let mut input = String::from("");
    stdin().read_line(&mut input).expect("Expect String or char");
    input = input.trim().to_ascii_lowercase();

    input = dbg!(input);
    if input.len()==1 {
        return Guess::CharGuess(input.chars().next().unwrap());
    } else {
        return Guess::StringGuess(input);
    }
}

