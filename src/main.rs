use std::io;
use rand::Rng;

// rock paper scissors game where you play against computer
fn main(){
    
    let options = ["rock", "paper", "scissors"];

    let index = rand::thread_rng().gen_range(0..=(options.len()-1));
    //let index = thread_rng().gen_range(options.len());

    let bot=options[index];



    println!("Welcome to the Game");
    println!("-------------------");
    println!("Make your choice: (r)ock, (p)aper, (s)cissors");

    let mut choice=String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Not valid selection");

    println!("you chose:{choice}");



    if choice.contains("r"){
        //println!("rock");
        let user=String::from("rock");

        println!("user: {user}");
        println!("bot: {bot}");

        if bot.contains("rock"){
            println!("Draw!");
        }

        else if bot.contains("paper"){
            println!("You Lose!");
        }

        else if bot.contains("scissor"){
            println!("You Win!");
        }
    }

    if choice.contains("p"){
        //println!("paper");

        //println!("paper");
        let user=String::from("paper");

        println!("user: {user}");
        println!("bot: {bot}");

        if bot.contains("paper"){
            println!("Draw!");
        }

        else if bot.contains("scissors"){
            println!("You Lose!");
        }

        else if bot.contains("rock"){
            println!("You Win!");
        }
    }

    if choice.contains("s"){
        //println!("scissors");

        //println!("scissors");
        let user=String::from("scissors");

        println!("user: {user}");
        println!("bot: {bot}");

        if bot.contains("scissors"){
            println!("Draw!");
        }

        else if bot.contains("rock"){
            println!("You Lose!");
        }

        else if bot.contains("paper"){
            println!("You Win!");
        }
    }

}