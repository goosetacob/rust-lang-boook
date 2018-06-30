extern crate rand;

use rand::Rng;

fn main() {
    println!("guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("input your guess.");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess)
            .expect("failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{} is too smol", guess),
            std::cmp::Ordering::Greater => println!("{} is too HUGE", guess),
            std::cmp::Ordering::Equal => {
                println!("you got it boiii/guuurlll");
                break;
            },
        }
    }
}
