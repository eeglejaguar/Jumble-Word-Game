use rand::{Rng, thread_rng};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use rand::seq::SliceRandom;

fn main() {

    println!("Welcome to Jmblue Word!");
    println!("You need to guess the word right to win!!");
    println!("Your Game Begins!!!");

    let file = File::open("words.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let random_item_number = rand::thread_rng().gen_range(0..lines.len());

    let random_word: &str = lines[random_item_number].as_str();

    println!("{:?}", jumble_word(random_word));

    println!("Guess the word");

    let mut guessed_word = String::new();

    let user_input = io::stdin();

    user_input.read_line(&mut guessed_word).expect("");


    if guessed_word.to_lowercase().trim() == random_word.to_lowercase() {
        println!("Yahoo! You guessed the word. ");
    } else {
        println!("Try again.\n GAME OVER!!!!!!");
    }
}


fn jumble_word(word: &str) -> String {
    let mut word_characters: Vec<char> = word.chars().collect();
    word_characters.shuffle(&mut thread_rng());
    word_characters.iter().collect()
}