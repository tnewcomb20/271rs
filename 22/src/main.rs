use std::fs::File;
use std::io::{self, BufRead, BufReader, Read}; 

// ANSI color codes
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

// Box drawing
const TOP: &str = "┌───┬───┬───┬───┬───┐";
const MID: &str = "├───┼───┼───┼───┼───┤";
const BOT: &str = "└───┴───┴───┴───┴───┘";

fn load_words(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let word = line?.trim().to_lowercase();
        if word.len() == 5 {
            words.push(word);
        }
    }

    Ok(words)
}

fn random_index(max: usize) -> usize {
    let mut byte = [0u8];
    File::open("/dev/random")
        .unwrap()
        .read_exact(&mut byte)
        .unwrap();
    (byte[0] as usize) % max
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

fn print_letter(c: char, color: &str) {
    print!("│ {}{}{} ", color, c, RESET);
}

fn render_guess(guess: &str, answer: &str) {
    for (i, gc) in guess.chars().enumerate() {
        let ac = answer.chars().nth(i).unwrap();
        if gc == ac {
            print_letter(gc, GREEN);
        } else if answer.contains(gc) {
            print_letter(gc, YELLOW);
        } else {
            print_letter(gc, RED);
        }
    }
    println!("│");
}

fn render_board(guesses: &[String], answer: &str) {
    clear_screen();
    println!("{}", TOP);
    for i in 0..guesses.len() {
        render_guess(&guesses[i], answer);
        if i != guesses.len() - 1 {
            println!("{}", MID);
        } else {
            println!("{}", BOT);
        }
    }
}

fn main() {
    let word_list = load_words("words.txt").expect("Failed to load word list");
    let answer = &word_list[random_index(word_list.len())];

    let mut guesses = vec!["     ".to_string(); 6];
    let stdin = io::stdin();
    let mut attempts = 0;

    println!("Use lowercase only btw.");

    while attempts < 6 {
        let mut guess = String::new();
        stdin.read_line(&mut guess).unwrap();
        guess = guess.trim().to_lowercase();

        if !word_list.contains(&guess) {
            println!("Not a valid word!");
            continue;
        }

        guesses[attempts] = guess.clone();
        render_board(&guesses, answer);

        if guess == *answer {
            println!("Winner!");
            return;
        }

        attempts += 1;
    }

    println!("Game over :( The word was: {}", answer);
}

