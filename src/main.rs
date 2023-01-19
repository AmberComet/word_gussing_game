use console::Term;
use inquire::{InquireError, Select};
use question::{Answer, Question};
use rand::Rng;
use std::fs::File;
use std::io;
use std::path::Path;

fn main() {
    let term = Term::stdout();
    // Build the start menu
    let options: Vec<&str> = vec![
        "All Words (Inculdes All ASCII Charicters)",
        "Only Words That Only Contains Alphabetic Charicters",
    ];
    //return the choice
    let ans: Result<&str, InquireError> =
        Select::new("What Word Set Do You Want To Use?", options).prompt();
    //unwrap the choice
    let file = match ans {
        Ok(choice) => choice,
        Err(_) => panic!(),
    };

    //put word bank into memory
    println!("Intalizing words");
    let words = words_initalization(choose_word_bank(file));

    //game loop
    loop {
        //intalize a game round
        //pull a random word from list of words
        let answer = &words[rand::thread_rng().gen_range(0..=words.len() - 1)].to_lowercase();

        //convert the word to vec
        let answer_char: Vec<char> = answer.chars().collect();

        //intalize a new empty vec to store user answers
        let mut user_char: Vec<char> =
            intialize_empty_answer(answer_char.len().try_into().unwrap());

        println!("the length of the word is {}", answer_char.len());

        println!("how many tries do you want");
        let mut tries = in_int();
        let tries_ref = tries;

        //this is for debugging
        //println!("{answer}");

        let mut used_hint = false;

        //guess loop
        while tries >= 0 {
            //hint
            if used_hint == false && (tries == tries_ref / 2) {
                let hint = Question::new("Do you want a hint?")
                    .default(Answer::YES)
                    .show_defaults()
                    .confirm();

                if hint == Answer::YES {
                    loop {
                        let hint_char = rand::thread_rng().gen_range(0..=answer_char.len() - 1);
                        if answer_char[hint_char] == user_char[hint_char] {
                            continue;
                        } else {
                            println!("The word contains {}", answer_char[hint_char]);

                            break;
                        }
                    }
                    used_hint = true;

                    //for debug
                    //println!("{used_hint}")
                }
            }

            //prompt and take user input
            print!("Enter a charicter ");
            let guess = Term::read_char(&term).expect("failed to read char");
            print!("{}\n", &guess);

            //create index value
            let mut i = 0;

            //check to see if the word contains the user guess
            if answer_char.contains(&guess) && !(user_char.contains(&guess)) {
                while i < user_char.len() {
                    if guess == answer_char[i] {
                        user_char[i] = guess
                    }

                    i = i + 1;
                }
            } else if user_char.contains(&guess) {
                println!("You Have Already Guessed {guess}\n You Have {tries} Guesses Left ")
            } else {
                println!("The word does not contain {guess}\n You Have {tries} Guesses Left ");
                tries = tries - 1;
            }
            print_vec(&user_char);

            //win condition
            if user_char == answer_char {
                println!("You Win!!\n The word was {answer}");
                break;

            //loose condition
            } else if (tries == 0) && (user_char != answer_char) {
                println!("You Lose :(\n The word was {answer}");
                break;

            //continue condtion
            } else {
                continue;
            }
        }

        //prompt for new round
        let answer = Question::new("Go again?")
            .default(Answer::YES)
            .show_defaults()
            .confirm();

        if answer == Answer::YES {
            continue;
        } else {
            break;
        }
    }
    println!("end of program");
}

//figures out what word bank to intalize
fn choose_word_bank(answer: &str) -> &str {
    if answer == "All Words (Inculdes All ASCII Charicters)" {
        return "words_alpha.txt";
    } else {
        return "words_alpha.txt";
    }
}

//intalize the word bank (not 100% sure how this works but it works (yes i stole it))
fn words_initalization(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Error: Cant Find File");
    let buf = io::BufReader::new(file);
    io::BufRead::lines(buf)
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

//creates a blank answer vec to fill into and user plays the game
fn intialize_empty_answer(len: u32) -> Vec<char> {
    let mut empty: Vec<char> = Vec::new();
    let mut i = 0;

    while i < len {
        empty.push('_');

        i += 1;
    }
    return empty;
}

//prints out any vectors i might have
fn print_vec(current: &Vec<char>) {
    for c in current {
        print!("{c}");
    }
    print!("\n");
}

//takes a line from std in and parses out any ints
fn in_int() -> i32 {
    //new empty string
    let mut x = String::new();

    //read next line from standard in
    io::stdin().read_line(&mut x).expect("Failed to read line");

    //parse the string to a u32
    let x: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    return x;
}