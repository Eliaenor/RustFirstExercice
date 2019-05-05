use std::thread::sleep;
use std::time::Duration;

pub fn lyrics() {
    const LYRICS: &[&str] = &[
        "A pizza with pepperoni !",
        "Two comic books",
        "Three skateboards",
        "Four man-hole covers",
        "Five video games, made it",
        "Six frisbees",
        "Seven silk'kimonos",
        "Eight chopsticks",
        "Nine narrow neckties",
        "Ten yellow yo-yos",
        "Eleven pairs of sneakers",
        "Twelve April O'Neil autographs",
    ];

    const DAY: &[&str] = &[
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    println!("Let's sing !");

    for (i, day_name) in DAY.iter().enumerate() {
        println!(
            "On the {} day of christmas the turtles gave to me",
            day_name
        );
        for index in (0..i + 1).rev() {
            if index == 0 && i > 0 {
                let mut final_line = String::from(LYRICS[index]);
                let first_char = final_line.remove(0).to_ascii_lowercase();
                final_line.insert(0, first_char);
                println!("and {}", final_line);
            } else {
                println!("{}", LYRICS[index]);
            }
        }
        println!();
        sleep(Duration::new(1, 0));
    }
}
