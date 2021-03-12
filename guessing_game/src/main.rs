extern crate rand;

use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o número!");

    loop {
        let secrect_number = rand::thread_rng()
            .gen_range(1..11);
    
            
        let mut guess = String::new();
            
        print!("Digite seu palpite » ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut guess)
            .expect("Houve uma falha ao ler seu palpite.");
    
        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        println!("Você digitou: {}", guess);
            
        match guess.cmp(&secrect_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou o número secreto.");
                break;
            }
        }
    }
}
