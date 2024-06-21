// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;
use std::collections::HashSet;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to Hangman!");
    let mut word_so_far = Vec::new();
    let mut guessed = HashSet::new();
    let mut not_guessed_words = HashSet::new();

    for i in 0..secret_word_chars.len() {
        word_so_far.push('-');
        not_guessed_words.insert(secret_word_chars[i]);
    }

    loop {
        print!("\x1B[2J\x1B[1;1H"); // ANSI转义序列，清空整个控制台并将光标移到左上角
        println!(
            "The word so far is {}",
            word_so_far.iter().collect::<String>()
        );
        println!(
            "You have guessed the following letters: {}",
            guessed.iter().collect::<String>()
        );
        // println!(
        //     "You have {} guesses left",
        //     NUM_INCORRECT_GUESSES - guessed.len() as u32
        // );
        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        let guess_char = guess.chars().nth(0).unwrap();
        if !guessed.contains(&guess_char) {
            guessed.insert(guess_char);
            if not_guessed_words.contains(&guess_char) {
                for i in 0..secret_word_chars.len() {
                    if secret_word_chars[i] == guess_char {
                        word_so_far[i] = guess_char;
                    }
                }
                not_guessed_words.remove(&guess_char);

                if not_guessed_words.len() == 0 {
                    println!("Congratulations you guessed the secret word: {}!", secret_word);
                    break;
                }
            } else {
                println!("Sorry, that letter is not in the word")
            }
        }
    }
}
