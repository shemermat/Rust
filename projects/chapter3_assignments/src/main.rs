use std::io;

fn main() {
    loop {
        println!("Pick what you want to do:");

        print_menu();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please pick a number");
                continue;
            }
        };

        match input {
            1 => temperature_converter(input),
            2 => temperature_converter(input),
            3 => generate_nth_fibbonacci(),
            4 => print_the_twelve_days_of_christmas(),
            _ => exit(),
        }
    }
}

fn print_the_twelve_days_of_christmas() {
    println!("The Twelve Days Of Christmas:");
    println!("");

    let lines = [
        "and a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "12 Drummers Drumming",
    ];
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for number in 0..12 {
        println!("On the {} day of Christmas", days[number]);
        println!("my true love sent to me:");

        if number == 0 {
            println!("A Partridge in a Pear Tree");
            println!(" ");
        } else {
            for num in (0..(number + 1)).rev() {
                println!("{}", lines[num]);
            }
            println!(" ");
        }
    }
}

fn generate_nth_fibbonacci() {
    let flag = true;

    while flag {
        let mut nth = String::new();

        println!("Please enter which fibbonacci number you want or '0' to quit");

        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line");

        let nth: u32 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if nth == 0 {
            println!("quitting");
            return;
        }

        let mut result = 0;

        if nth < 3 {
            println!("The {} in fibbonacci sequence is 1", nth);
            continue;
        }

        let mut iter = 3;
        let mut num1 = 1;
        let mut num2 = 1;

        while iter <= nth {
            result = num1 + num2;
            if iter == 3 {
                num2 = result;
            } else {
                if num1 < num2 {
                    num1 = result;
                } else {
                    num2 = result;
                }
            }

            iter = iter + 1;
        }

        println!("The {} ficonnacci number is {}", nth, result);
    }
}

fn exit() {
    println!("Exiting the program!");
    std::process::exit(1);
}

fn print_menu() {
    println!("1. Convert from Celsius to Fahrenheit");
    println!("2. Convert from Fahrenheit to Chlsius");
    println!("3. Generate Fibonacci number");
    println!(r#"4. Print the lyrics of the Christmes carol "The Twelve Days of Christmas""#);
    println!("5. Exit");
}

fn temperature_converter(pick: u32) {
    loop {
        if pick == 1 {
            println!(
                "Please enter the temperature you want to convert in Celsius or 1.99999 to quit"
            );
        } else {
            println!(
                "Please enter the temperature you want to convert in Fahrenheit or 1.99999 to quit"
            );
        }

        let temp = 0;
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if (temp == 1.99999) {
            println!{"quitting...."}
            return;
        }

        let mut newtemp = 0.1;

        if pick == 1 {
            newtemp = (temp * 1.8) + 32.0;
        } else {
            newtemp = (temp - 32.0) / 1.8;
        }

        println!("The new temperature is: {}", newtemp);
    }
}
