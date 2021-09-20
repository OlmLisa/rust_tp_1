use rand::Rng;
use std::io;

fn main() {
    let num = rand::thread_rng().gen_range(0..5);
    println!("Bienvenue sur guess my number !! ");
    loop {
        println!("Choisissez un nombre entre 0 et 5 :  ");
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
    
        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => println!("Votre choix est : {}", i),
            Err(..) => println!("This was not an integer: {}", trimmed),
        };
        let choice = trimmed.parse::<i32>().unwrap();
        if choice < 0 {
            println!("You enter a negative value!! ");
            continue;
        }
        if choice < num {
            println!("Le nombre secret est plus grand que {}", choice);
        }
        else if choice > num {
            println!("Le nombre secret est plus petit que {}", choice);
        }
        else{
            println!("Bravo !");
            break;
        }
    }
}
