fn main() {
    let mut message = String::from("Hello");
    message = extend_message(message);

    let age = 30;
    extend_age(age);
    println!("{}", age);

    println!("new_message: {}", message);
}

fn extend_message(mut a: String) -> String {
    a.push_str("World");
    a
}

fn extend_age(mut a: u32) -> u32 {
    a += 100;
    a
}
