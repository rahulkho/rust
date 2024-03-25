use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){

    println!("Guess The Number !");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The Secret Number is {secret_number}");

    

    loop {

        println!("\nPlease Input Your Guess : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To Read Line");
        
        //let guess: u32 = guess.trim().parse().expect("Please Type Integer");

        let guess: u32 = match guess.trim().parse( ){
            Ok(num)=> num,
            Err(_)=> { 
                print!( "Please Type A Number \n");
                continue; 
            },
        };

        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Your number is greater!"),
            Ordering::Less  => println!("Your number is smaller!"),
            Ordering::Equal => { 
                println!("You Win");
                break;
            },
        }
    }
    

}