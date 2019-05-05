use std::io;
mod christmas_turtle;
mod degree_converter;
mod fibonacci;

fn main() {
    println!(
        "Choose a Game :\
         \n 1 - F° to C° \
         \n 2 - Get the Nth Fibonacci Number \
         \n 3 - Get lyrics of 'The Twelve Days of Christmas' by TMNT\
         \n\n\
         Type 0 to exit"
    );

    let mut game_chosen = String::new();

    loop {
        io::stdin()
            .read_line(&mut game_chosen)
            .expect("You must choose a game by inputing 0, 1, 2 or 3.");

        match game_chosen.trim().parse() {
            Ok(num) => match num {
                0 => break,
                1 => degree_converter::convert(),
                2 => fibonacci::play(),
                3 => christmas_turtle::lyrics(),
                _ => println!("You must input a number from 1 to 3 only !"),
            },
            Err(_) => {
                println!("You must input a number from 0 to 3 !");
                continue;
            }
        };
        break;
    }

    println!("Game finished");
}
