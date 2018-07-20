use std::io;
use std::env;


fn main() {
    let mut input = String::new();
    let mut mock = String::new();

    let args: Vec<String> = env::args().collect();

    input = match args.get(1) {
        Some(value) => value.to_string(),
        None => {
            println!("What would you like to mock?");

            io::stdin().read_line(&mut input)
                .expect("fAiLeD tO rEaD LiNe");
            input
        },
    };

    let input = input.trim().to_ascii_lowercase();

    let mut is_uppercase = false;

    for c in input.chars() {
        if c.is_alphabetic() & is_uppercase {
            mock.push(c.to_uppercase().collect::<Vec<_>>()[0]);
            is_uppercase = false;
        } else {
            mock.push(c);
            if c.is_alphabetic() {
                is_uppercase = true;
            }
        }
    }

    println!("{}", mock)
}
