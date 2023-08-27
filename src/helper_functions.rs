use std::{fs, io};
use rand::seq::SliceRandom;
use rand::thread_rng;


const DICTIONARY_PATH: &str = "Dictionary.txt";

pub fn open_dictionary() -> Result<String, io::Error>{
    Ok(fs::read_to_string(DICTIONARY_PATH)?)
}

pub fn random_word(len: &usize) -> Result<String, io::Error>{
    let dictionary = open_dictionary()?;
    let dictionary: Vec<&str> = dictionary.split_ascii_whitespace().into_iter().filter(|x| x.len()==*len).collect();
    Ok(dictionary.choose(&mut thread_rng()).unwrap().to_string().to_ascii_lowercase())
}

#[cfg(test)]
mod random_word {
    use crate::helper_functions::random_word;

    #[test]
    #[should_panic] // for normal scrabble dictionary. need to change for other dictionaries
    fn is_1_letters() {
        let _ = random_word(&1).unwrap().len();
        // assert_eq!(result, 5);
    }

    #[test]
    fn is_2_letters() {
        let result = random_word(&2).unwrap().len();
        assert_eq!(result, 2);
    }

    #[test]
    fn is_not_4_letters() {
        let result = random_word(&5).unwrap().len();
        assert_ne!(result, 4);
    }

    #[test]
    fn is_5_letters() {
        let result = random_word(&5).unwrap().len();
        assert_eq!(result, 5);
    }

    #[test]
    fn is_15_letters() {
        let result = random_word(&15).unwrap().len();
        assert_eq!(result, 15);
    }

    #[test]
    #[should_panic] // for normal scrabble dictionary. need to change for other dictionaries
    fn is_16_letters() {
        let _ = random_word(&16).unwrap().len();
        // assert_eq!(result, 5);
    }

    
}




