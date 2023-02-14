use std::io;
use std::cmp::Ordering;
use rand::Rng; // Rng defines random number generator

fn main() {
    println!("Guess the number!");

    // thread_rng function that gives us the particular random number generator
    // gen_range function that generates a random number between two numbers 1 to 100 or whatever you pass in
    let secret_number = rand::thread_rng().gen_range(1..=1000);

    // println!("The secret number is: {secret_number}");
loop {

    println!("Please input your guess.");

    let x = 5;
    let y = 10;
    let mut guess = String::new();
    // let mut bananas = 5; // mutable


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");
    println!("x = {} and y = {}", x, y);

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
println!("You win!");
            break;
    }
}
}
}
