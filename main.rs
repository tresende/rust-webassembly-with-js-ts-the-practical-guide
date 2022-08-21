fn main() {
    let message = "Hello World";
    print_welcome(message);
}

fn print_welcome(text: &str) -> &str {
    println!("Message: {}", text);

    ""
}
