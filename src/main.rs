use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("I have a number, please share your guess:");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        let guess: i32 = match input.trim().parse() {
              Ok(num) => num,
              Err(_) => continue,
            };
          
        match guess.cmp(&secret_number){
            Ordering::Equal => {
              println!("Spot on!");
              break;
            }
            Ordering::Less => println!("Come on, {} is too low!", guess),
            Ordering::Greater => println!("{}...too high!", guess),
        }
    }
}
