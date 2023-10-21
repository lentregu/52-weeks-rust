use std::io;

// This version of guessing_game handles error in order to avoid the program to crash
// in the event of an error when reading the user input.

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    // We use a match expression to handle the Result returned by read_line. If read_line returns Ok(_), 
    // it means the input was successfully read, and we proceed to use the input. 
    // If it returns an Err, we print an error message. 
    match io::stdin().read_line(&mut guess) {
        Ok(bytes_read) => {
            println!("You guessed: {}", guess);
            println!("Bytes read: {}", bytes_read);
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
        }
    }

    another_way_without_using_match()
}

fn another_way_without_using_match() {
    println!("This function to the same guessing game, but this way don't use match");
    
    println!("Please input your guess.");

    let mut guess = String::new();

    let result = io::stdin().read_line(&mut guess);

    // Instead of using match we call the is_ok() method to know if the result is Ok.
    if result.is_ok() {
        println!("You guessed: {}", result.unwrap());
    } else {
        eprintln!("Error reading input: {:?}", result.unwrap_err());
    }
}