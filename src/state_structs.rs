use std::collections;
use std::string;
use std::vec;

use crate::game_helper_functions::Guess;

#[derive(Clone)]
#[derive(Debug)]
pub enum CharState{
    Char(char),
    Blank,
}

pub enum GuessEnum{
    CorrectLetter,
    CorrectWord,
    IncorrectLetter,
    IncorrectWord,
    AlreadyGuessed,
}

#[derive(Debug)]
pub struct GuessState{
    pub string_state: Vec<CharState>,
    pub letters_guessed: collections::HashSet<char>,
    pub words_guessed: Vec<String>,
}

impl GuessState {
    fn new(word_to_guess_length: usize) -> Self {
        Self {
            string_state: vec![CharState::Blank; word_to_guess_length],
            letters_guessed: collections::HashSet::new(),
            words_guessed: Vec::new(),
        }
    }

    fn in_guessed(&self , char: char) -> bool{
        self.letters_guessed.contains(&char)
    }

    fn update_char_game_state(&mut self, char: char, char_index: Vec<usize>){
        //let _ = dbg!(&char_index.iter().map(|&x| {self.string_state[x] = CharState::Char(char)}));
        for x in char_index{
            self.string_state[x] = CharState::Char(char)
        }
        self.letters_guessed.insert(char);
    }

    fn add_guessed_word(&mut self, word: &String){
        self.words_guessed.push(word.to_string());
    }

    fn to_string(&self) -> String{
        let mut temp_string = String::new();
        // let _ = self.string_state.iter().map(|char_state| temp_string.push(
        //     match char_state{
        //         CharState::Char(c) => *c,
        //         CharState::Blank => '_',
        //     }
        // ));
        for x in &self.string_state{
            temp_string.push(
                match x {
                    CharState::Char(c) => *c,
                    CharState::Blank => '_',
                }
            )
        }
        temp_string
    }

    pub fn number_attemps(&self) -> usize {
        self.letters_guessed.len() + self.words_guessed.len()
    }
}

#[derive(Debug)]
pub struct GameState{
    pub word_to_guess: String,
    pub current_state: GuessState,
}

impl GameState{
    pub fn new(word_to_guess: &String) -> Self{

        Self{
            word_to_guess: word_to_guess.clone(),
            current_state: GuessState::new(word_to_guess.len())
        }
    }

    pub fn try_guess(&mut self, guess: &Guess) -> bool{
        let guess_results = match guess {
            Guess::CharGuess(char) => self.try_letter(char),
            Guess::StringGuess(string) => self.try_word(string)
        };

        if matches!(guess_results, GuessEnum::CorrectWord){
            return true
        }

        if self.is_win() {return true}

        self.print_guess_results(&guess_results);
        false
    }

    fn try_letter(&mut self, char: &char) -> GuessEnum{
        let char = char.to_ascii_lowercase();
        if self.current_state.in_guessed(char){ //letter is guessed before
            return GuessEnum::AlreadyGuessed;
        }

        self.current_state.update_char_game_state(char, self.get_positions(char));//to change

        if self.word_to_guess.contains(char){ //letter in word
            return GuessEnum::CorrectLetter; 
        }

        GuessEnum::IncorrectLetter
    }

    fn try_word(&mut self, guessed_word: &String) -> GuessEnum{
        if self.current_state.words_guessed.contains(guessed_word){ //word is guessed before
            return GuessEnum::AlreadyGuessed;
        }

        self.current_state.add_guessed_word(&guessed_word);

        if guessed_word.eq(&self.word_to_guess) { //word guessed is correct
            return GuessEnum::CorrectWord
        }

        GuessEnum::IncorrectWord
    }

    fn print_guess_results(&self, guess_results: &GuessEnum){
        println!("{}", 
            match guess_results {
                GuessEnum::CorrectLetter => "Letter exist",
                GuessEnum::CorrectWord => "Correct word", //to be removed
                GuessEnum::IncorrectLetter => "No such letter",
                GuessEnum::IncorrectWord => "Wrong word",
                GuessEnum::AlreadyGuessed => "Already Guessed"
        });
    }


    fn get_positions(&self, char: char) -> Vec<usize>{
        self.word_to_guess.match_indices(char).map(|(i, _)|i).collect()
    }

    fn is_win(&self) -> bool{
        self.current_state.to_string().eq_ignore_ascii_case(&self.word_to_guess)
    }

    pub fn print_state(&self){
        let mut state: String = String::new();
        for x in &self.current_state.string_state{
            match x {
                CharState::Char(char) => state.push(*char),
                CharState::Blank => state.push('_')
            }
            state.push(' ');
        };

        println!("{state}");
    }
}


#[cfg(test)]
mod test_game_state {
    #[test]
    fn it_works() {
        todo!("Need test structs, not sure how");
    }
}
