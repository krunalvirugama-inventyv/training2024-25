use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number Game !");

  

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret Number = {secret_number}" , );

    loop{
        println!("Please input your guess.");
        let mut number = String::new();
    
        io::stdin()
            .read_line(&mut number)
            .expect("Failed read line");

        let number: usize = number
            .trim()
            .parse()
            .expect("Enter was not a number");


        if secret_number == number {
            println!("You Are Perfact Right ! " );
            break;
        }

        if number > secret_number {
            println!("Your guess is High" );
        }else {
            println!("Your guess is Low" );
        }

    }
   

   
}
