use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("I have a number guess what it is?");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to parse input");

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}??? Too Small", guess),
            Ordering::Equal => {
                println!("Spot on!");
                break;
            }
            Ordering::Greater => println!("Too much, champ!"),
        }
    }
}
