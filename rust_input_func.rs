use std::io::{self, Write};

fn main() {
    let x = input("Enter something");
    println!("{}", x);
}

fn input(promp_msg: &str) -> String {
    println!("{}", promp_msg);
    print!("> ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line.");
    s.trim().to_string()
}
