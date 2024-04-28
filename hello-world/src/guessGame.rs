use std::io; //import del parquete input output

fn main(){ //punto de entrada del programa main

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //mut hace referencia a que una variable puede cambiar de valor

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Yout guessed: {guess}");
    
}