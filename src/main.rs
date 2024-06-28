use std::io;

fn main() {
    // welcome message
    println!("Hi, I'm a Rust chatbot! How can I help you today?");

    loop {
        let mut query = String::new();
        io::stdin()
            .read_line(&mut query)
            .expect("Failed to read line");

        println!("You said: {}", query.trim());
    }
}
