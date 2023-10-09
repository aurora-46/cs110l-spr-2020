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
use std::iter::FromIterator;

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
    let mut guess_word_chars:Vec<char>=secret_word_chars.iter().map(|_|'-').collect();
    let mut guess_word_process: Vec<char>=Vec::new();
    println!("Welcome to CS110L Hangman!");

    let mut guess_count=5;

    while guess_count>0 &&guess_word_chars!=secret_word_chars{
        println!("The word so far is {}",String::from_iter(guess_word_chars.iter()));
        println!("You have guessed the following letters: {}",String::from_iter(guess_word_process.iter()));
        println!("You have {guess_count} guesses left");
        print!("Please guess a letter: ");
        let _=io::stdout().flush();

        let mut input = String::new();
        if let Err(_)=io::stdin().read_line(&mut input){
            println!("");
            continue;
        };

        let guess_char:Vec<char>=input.chars().collect();
        let guess_char:char=guess_char[0];
        guess_word_process.push(guess_char);

        let mut find=false;
        for (x,&y)in secret_word_chars.iter().enumerate(){
            if y==guess_char&&guess_word_chars[x]!=guess_char{
                find=true;
                guess_word_chars[x]=guess_char;
                break;
            }
        }
        if !find {
            guess_count-=1;
            println!("Sorry, that letter is not in the word");

        }
        println!("");
        continue;
    }
    if guess_word_chars==secret_word_chars{
        println!("Congratulations you guessed the secret word: {}!",secret_word);
    }
    else{
        println!("Sorry, you ran out of guesses!");
    }
}