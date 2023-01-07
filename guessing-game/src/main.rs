use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_num}");

    // use a loop to have user guess until correct 
    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert the user input to a 32-bit integer 
        let guess: u32 = match guess.trim().parse() { 
            Ok(num) => num,
            Err(_) => continue,
        } ;
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
