use clap::Parser;

mod helper_functions;

mod state_structs;
mod human_game;
mod game_helper_functions;


/// A program to play hangman
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of word
    #[arg(short, long, default_value_t = 4)]
    length: u8,

    /// Word to use
    #[arg(short, long, default_value = None)]
    word: Option<String>,

    // Number of human rounds
    #[arg(short, long, default_value_t = 1)]
    rounds: i32,

    // AI to participate
    #[arg(short, long, default_value_t = true)]
    ai: bool,
}

fn main() {
    let args = Args::parse();
    dbg!(&args);

    //currently user input word may not be valid
    let word: String = args.word.unwrap_or(helper_functions::random_word(&(args.length as usize)).unwrap());

    human_game::run_human_games(&word, args.rounds);
}