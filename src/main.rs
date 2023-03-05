use rand::Rng;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    io,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Correctness {
    Correct,
    Incorrect,
    Misplaced,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Validation {
    Valid,
    NotWord,
    Not5Letters,
}

fn main() {
    let answers = get_answers("C:/Users/grand/dev/learning_rust/wordle/words/answers.txt");
    let answer = answers[rand::thread_rng().gen_range(1..=answers.len())].clone();
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

        if guess.trim() == answer.trim() {
            println!("You win!");
            std::process::exit(0);
        }

        if guess.trim().len() != 5 {
            println!("Not 5 letters");
            continue;
        } /*else if !answers.contains(&guess) {
            println!("Not a word");
            continue;
        }*/ else {
            println!("Valid");
        }

        evaluate(answer.as_str(), guess.as_str(), count);
        println!("{:?}", evaluate(answer.as_str(), guess.as_str(), count));

        count += 1;
    }
}

fn evaluate(answer: &str, guess: &str, count: i32) -> Vec<Correctness> {
    let mut checker = vec![Correctness::Incorrect; 5];
    let mut accounted_for: Vec<usize> = Vec::new();

    for (idx, (g, a)) in guess.bytes().zip(answer.bytes()).enumerate() {

        if g == a {
            checker[idx] = Correctness::Correct;
            accounted_for.push(idx as usize);
        } else if answer.contains(g as char) {
            let repeats = check_for_repeats(String::from(answer), g as char);
            for i in repeats {
                if !accounted_for.contains(&(i as usize)) {
                    checker[i as usize] = Correctness::Misplaced;
                    accounted_for.push(i as usize);
                }
            }
            
        }
    }

    if count > 0 {
        checker.clear();
    }


    checker
}

fn get_answers(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn check_for_repeats(guess: String, letter: char) -> Vec<i32> {
    let mut indices: Vec<i32> = Vec::new();

    for (idx, l) in guess.chars().enumerate() {
        if l == letter {
            indices.push(idx as i32);
        }
    }

    indices
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
    fn test_check_for_repeats() {
        assert_eq!(check_for_repeats(String::from("hello"), 'l'), vec![2, 3]);
        assert_eq!(check_for_repeats(String::from("hello"), 'h'), vec![0]);
        assert_eq!(check_for_repeats(String::from("hello"), 'o'), vec![4]);
    }
}
