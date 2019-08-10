use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
  println!("Guess the number!");

  let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

  // println!("The secret number is: {}", secret_number);

  loop {
    println!("Please enter an integer between 1 and 100...");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line!");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Equal => {
          println!("Congratulations, you guessed it!");
          break;
        }, 
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!"),
    }
  }
}
