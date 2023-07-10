use std::io::{stdin, stdout, Write};

fn main() {
    let secret_word = "wanton".to_string();
    let mut guesses: Vec<String> = Vec::new();

    loop {
        println!();
        for guess in &guesses {
            println!("[ {} ]", guess);
        }
        println!();

        print!("> ");
        stdout().flush().unwrap();

        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Error in reading input");

        let guess = guess.trim().to_string();

        let guess_len = &guess.chars().count();
        let secret_word_len = &secret_word.chars().count();

        let valid_word_len = guess_len == secret_word_len;
        let correct = guess == secret_word;

        if correct {
            break;
        } else if valid_word_len {
            guesses.push(guess);
        } else {
            println!("\nInvalid input");
        }
    }
}
