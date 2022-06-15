use std::io;

fn main() {
    println!("What is your name?");
    let mut person_name = String::new();
    io::stdin()
        .read_line(&mut person_name)
        .expect("Failed to read name!");
    let formatted_person_name = person_name.trim_end();
    println!("Hello, {formatted_person_name}. >:F");
}
