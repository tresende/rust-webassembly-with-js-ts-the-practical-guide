fn main() {
    let mut message = String::from("Hello");
    let message3 = &message;

    let message2 = &mut message; // & -> borrowed reference

    unpredictable_mutate(message2);
    println!("{}", message2);

    println!("{}", message);
}

fn unpredictable_mutate(val: &mut String) {
    val.push_str("_unpredictable")
}
