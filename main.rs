fn main() {
    let message = "Hello World";
    let message_2 = format_message(message);
    println!("{}", message_2)
}

fn format_message(text: &str) -> &str {
    let new_message = format!("message: {}", text);

    //return "Hi there!";
    Box::leak(new_message.into_boxed_str())
}
