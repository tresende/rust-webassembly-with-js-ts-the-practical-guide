fn main() {
    let message = String::from("Hello");
    let new_message = extend_message(message);
    println!("new_message: {}", new_message);
}

fn extend_message(mut a: String) -> String {
    a.push_str("World");
    a
}
