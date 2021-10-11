
use std::io;


fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

}

fn test(){
    println!("test2");
    let mut count = 1;
    while count < 10 {

        count += 1;
    }
}


