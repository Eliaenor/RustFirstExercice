use std::io;

pub fn convert() {
    println!("What do you want to do ?\n 1 - Celcius to Fahrenheit \n 2 - Fahrenheit to Celcius");

    loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("You must input 1 or 2");

        match choice.trim().parse() {
            Ok(num) => match num {
                1 | 2 => convert_degree(num),
                _ => {
                    println!("You must input either 1 or 2 only !");
                    continue;
                }
            },
            Err(_) => {
                println!("You must input either 1 or 2 only !");
                continue;
            }
        };
        break;
    }
}

fn convert_degree(choice: i32) {
    let degree_value = get_degree_value();

    match choice {
        1 => println!(
            "Your result is: {}°C is equal to {}°F",
            degree_value,
            (degree_value * 9.0 / 5.0) + 32.0
        ),
        2 => println!(
            "Your result is: {}°F is equal to {}°C",
            degree_value,
            (degree_value - 32.0) * 5.0 / 9.0
        ),
        _ => println!("You must input a 1 or a 2 only !"),
    };
}

fn get_degree_value() -> f64 {
    println!("Please enter you degree value : ");

    loop {
        let mut degree_value = String::new();

        io::stdin()
            .read_line(&mut degree_value)
            .expect("You must enter a number between -100 to +100");

        let degree_value: f64 = match degree_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You must enter a valid number between -100 and 100");
                continue;
            }
        };
        return degree_value;
    }
}
