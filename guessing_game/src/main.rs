use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let word = &String::from("hello world");
    println!("{}",first_word(word)) ;
}

fn guess_game() {
    println!("Guess the number!");
    /*
    In Rust, variables are immutable by default,
    meaning once we give the variable a value, the value won’t change.
    */
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    let mut guess: String = String::new();
    loop {
        println!("Please input your guess.");
        // clean up guess for new input
        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("guess: {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
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

fn return_number() -> i32 {
    return 5;
}
fn using_if_in_a_let() -> i32 {
    let number = if true { 5 } else { 6 };
    return number;
}

// Loop Labels to Disambiguate Between Multiple Loops
fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn sample_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
