fn main() {
    let mut message = String::from("Hello");
    message = extend_message(message);
    println!("new_message: {}", message);
}

fn extend_message(mut a: String) -> String {
    a.push_str("World");
    a
}
