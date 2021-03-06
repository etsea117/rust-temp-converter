use std::io;

fn main() {
    'converting: loop {
        println!("Input your number to convert");
        let mut conv_input = String::new();
        let success: bool;
        io::stdin()
            .read_line(&mut conv_input)
            .expect("Failed to read line");
        let conv_input: f64 = match conv_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        success = true;
        while success == true {
            let mut f_or_c = String::new();
            println!("What is your number? 'f' for Fahrenheit, 'c' for Celsius");
            io::stdin()
                .read_line(&mut f_or_c)
                .expect("Failed to read line");
            break 'converting match f_or_c.trim() {
                "f" => {
                    let cels = fahr_to_cels(conv_input);
                    println!("{} convertered to Celsius is {}", conv_input, cels);
                }
                "c" => {
                    let fahr = cels_to_fahr(conv_input);
                    println!("{} convertered to Fahrenheit is {}", conv_input, fahr);
                }
                "q" | "quit" => {
                    break 'converting;
                }
                other => {
                    println!("'{}' is not a valid option!", other);
                    continue;
                }
            };
        }
    }
}

fn cels_to_fahr(temp: f64) -> f64 {
    (temp * 9.0 / 5.0) + 32.0
}

fn fahr_to_cels(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}
