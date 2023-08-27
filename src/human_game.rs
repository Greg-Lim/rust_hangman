use crate::{state_structs, game_helper_functions::get_input};

pub fn run_human_games(word: &String, number_of_rounds: i32){
    for _ in 0..number_of_rounds{
        run_human_game(&word);
    }
}

fn run_human_game(word: &String){
    let mut game_state = state_structs::GameState::new(&word);
    // let words = game_helper_functions::open_dictionary().unwrap();

    loop{
        let guess = get_input();
        let is_win = game_state.try_guess(&guess);
        if is_win {
            println!("Win");
            break;
        }
        game_state.print_state();
    }

    println!("The word is {}, You took {} guesses", game_state.word_to_guess, game_state.current_state.number_attemps());
    // ^ this is wrong
}