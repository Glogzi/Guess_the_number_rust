#![allow(unused)]
use core::num;
use std::io::{self, Read};
use rand::Rng;

fn main(){
    loop {//main loop
        let random_number: u8 = rand::thread_rng().gen_range(1..=100);
        let mut num_ready = 0;
        loop {//loop for guessing a number + checking is it right
            loop {//loop for guessing a number
                println!("Guess a number in range 0-100");
                let mut num: String = String::new();
                io::stdin().read_line(&mut num)
                    .expect("ERROR, Wrong input");
    
                match num.trim().parse::<u8>() {
                    Ok(num) => {
                        if (num > 100) || (num < 1) {
                            println!("number you have chosen is out of range!!!");
                            continue;
                            
                        }else {
                            println!("number you have chosen is {}", num);
                            num_ready = num;
                            break;
                        }
                    }
                    Err(num) => {
                        println!("number you have chosen is out of range, or even is not a number!!!");
                        continue;
                    }
                }
            }
            if random_number == num_ready {
                println!("congratulation, your guess was correct! the number was {}", num_ready);
                break;
            }else if num_ready < random_number && num_ready != 0 {
                println!("nope, your number is too low");
            }else if num_ready > random_number{
                 println!("nope, your number is too high")}
        }
    }
}

