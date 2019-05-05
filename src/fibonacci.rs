use std::io;

pub fn play() {
    println!("Type the number of the Fibonacci list element you want :");

    loop {
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("You must input a valid positive number.");

        match number.trim().parse() {
            Ok(num) => {
                if num > 0 {
                    get_element(num)
                } else {
                    println!("You must input a valid positive number.");
                    continue;
                }
            }
            Err(_) => {
                println!("You must input a valid positive number.");
                continue;
            }
        };
        break;
    }
}

fn get_element(number: i32) {
    let mut first = 0;
    let mut second = 1;
    let mut result = 0;

    for _index in 1..number {
        result = first + second;
        first = second;
        second = result;
    }

    println!(
        "The {} element of the Fibonacci suite is : {}",
        number, result
    );
}
