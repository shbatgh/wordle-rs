#![allow(unused)]

use rand::Rng;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    io,
};
use colored::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Correctness {
    Correct,
    Incorrect,
    Misplaced,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Validation {
    Valid,
    NotWord,
    Not5Letters,
}

fn main() {
    let answers = get_answers("C:/Users/grand/dev/learning_rust/wordle/words/answers.txt");
    let guesses = get_answers("C:/Users/grand/dev/learning_rust/wordle/words/guesses.txt");
    //let answer = answers[rand::thread_rng().gen_range(1..=answers.len())].clone();
    let answer = String::from("polar");
    let mut count = 0;

    println!("Welcome to Wordle! You have 5 guesses to guess the word. Good luck!");
    println!("{answer}");

    loop {
        let mut guess = String::new();

        if count > 5 {
            println!("You lose! The word was: {}", answer);
            break;
        }

        println!("Guess the word: ");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess.trim();

        if guess.trim() == answer.trim() {
            println!("You win!");
            std::process::exit(0);
        }

        if guess.trim().len() != 5 {
            println!("Not 5 letters");
            continue;
        } /*else if !guesses.contains(&String::from(guess.trim())) {
            println!("Not a word");
            continue;
        }*/ else {
            println!("Valid");
        }

        evaluate(answer.as_str(), guess.as_str(), count);
        print_correctness(evaluate(answer.as_str(), guess.as_str(), count), guess.clone());
        println!("{:?}", evaluate(answer.as_str(), guess.as_str(), count));

        count += 1;
    }
}

fn evaluate(answer: &str, guess: &str, count: i32) -> Vec<Correctness> {
    let mut checker = vec![Correctness::Incorrect; 5];

    let mut unaccounted_for = [0; (b'z' - b'a' + 1) as usize];
    //let mut account_for_repeats: Vec<char> = Vec::new();

    for (idx, (g, a)) in guess.trim().bytes().zip(answer.bytes()).enumerate() {
        if g == a {
            checker[idx] = Correctness::Correct;
        } else {
            unaccounted_for[(a - b'a') as usize] += 1;
        }
    }

    for (idx, g) in guess.trim().bytes().enumerate() {
        if checker[idx] == Correctness::Correct {
            continue;
        }

        if unaccounted_for[(g - b'a') as usize] > 0 {
            checker[idx] = Correctness::Misplaced;
            unaccounted_for[(g - b'a') as usize] -= 1;
        }
    }
        
        /*else if answer.contains(g as char) {
            /*let repeats = check_for_repeats(String::from(answer.trim()), g as char);
            let idx_at_answer = &answer.find(g as char).unwrap();

            if !accounted_for.contains(&(idx as usize)) {
                checker[idx as usize] = Correctness::Misplaced;
                accounted_for.push(idx as usize);
                account_for_repeats.push(i as usize);
            }*/
            /*for i in repeats {
                
            }*/
            
            for i in 0..answer.len() {
                if answer.as_bytes()[i]  == g && !accounted_for.contains(&idx) {
                    println!("{}", answer.as_bytes()[i]);
                    checker[i] = Correctness::Misplaced;
                    accounted_for.push(idx);
                }
            }
        }*/

    checker
}

fn get_answers(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn check_for_repeats(answer: String, letter: char) -> Vec<i32> {
    let mut indices: Vec<i32> = Vec::new();

    for (idx, l) in answer.chars().enumerate() {
        if l == letter {
            indices.push(idx as i32);
        }
    }

    indices
}

fn print_correctness(correctness: Vec<Correctness>, guess: String) {
    for (idx, c) in correctness.iter().enumerate() {
        match c {
            Correctness::Correct => print!("{}", (guess.as_bytes()[idx] as char).to_string().green()),
            Correctness::Incorrect => print!("{}", (guess.as_bytes()[idx] as char).to_string().red()),
            Correctness::Misplaced => print!("{}", (guess.as_bytes()[idx] as char).to_string().yellow()),
        }
    }

    println!();
}

// Implement the .contains() method for Vec<i32> and return the indice of the letter
fn contains_return_indices() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn general_testing() {
        assert_eq!(evaluate("hello", "hello", 0), vec![Correctness::Correct, Correctness::Correct, Correctness::Correct, Correctness::Correct, Correctness::Correct]);
        assert_eq!(evaluate("hello", "hella", 0), vec![Correctness::Correct, Correctness::Correct, Correctness::Correct, Correctness::Correct, Correctness::Incorrect]);
        assert_eq!(evaluate("hello", "helle", 0), vec![Correctness::Correct, Correctness::Correct, Correctness::Correct, Correctness::Correct, Correctness::Incorrect]);
    }

    #[test]
    fn repeats() {
        assert_eq!(evaluate("polar", "hello", 0), vec![Correctness::Incorrect, Correctness::Incorrect, Correctness::Correct, Correctness::Incorrect, Correctness::Misplaced]);
    }

    #[test]
    fn all_wrong() {
        assert_eq!(evaluate("hello", "xxxxx", 0), vec![Correctness::Incorrect, Correctness::Incorrect, Correctness::Incorrect, Correctness::Incorrect, Correctness::Incorrect]);
    }

    #[test]
    fn all_correct() {
        assert_eq!(evaluate("hello", "hello", 0), vec![Correctness::Correct, Correctness::Correct, Correctness::Correct, Correctness::Correct, Correctness::Correct]);
    }

    #[test]
    fn all_misplaced() {
        assert_eq!(evaluate("hello", "olehl", 0), vec![Correctness::Misplaced, Correctness::Misplaced, Correctness::Misplaced, Correctness::Misplaced, Correctness::Misplaced]);
    }

    #[test]
    fn misplaced_and_correct() {
        assert_eq!(evaluate("hello", "hlelo", 0), vec![Correctness::Correct, Correctness::Misplaced, Correctness::Misplaced, Correctness::Correct, Correctness::Correct]);
    }

    #[test]
    fn same_letter_misplaced_twice() {
        assert_eq!(evaluate("hello", "hleol", 0), vec![Correctness::Correct, Correctness::Misplaced, Correctness::Misplaced, Correctness::Misplaced, Correctness::Misplaced]);
    }

    #[test]
    fn papa_bozo() {
        assert_eq!(evaluate("rrara", "rarrt", 0), vec![Correctness::Correct, Correctness::Misplaced, Correctness::Misplaced, Correctness::Correct, Correctness::Incorrect]);
    }

    #[test]
    fn papa_bozo2() {
        //println!("{:?}", evaluate("rrara", "abase", 0));
        assert_eq!(evaluate("rrara", "abase", 0), vec![Correctness::Misplaced, Correctness::Incorrect, Correctness::Correct, Correctness::Incorrect, Correctness::Incorrect]);
    }

    #[test]
    fn test_check_for_repeats() {
        assert_eq!(check_for_repeats(String::from("hello"), 'l'), vec![2, 3]);
        assert_eq!(check_for_repeats(String::from("hello"), 'h'), vec![0]);
        assert_eq!(check_for_repeats(String::from("hello"), 'o'), vec![4]);
    }
}
