fn main() {
    let mut message = String::from("Hello");
    let message2 = &mut message; // & -> borrowed reference

    message2.push_str(" world");

    println!("{}", message2);
    println!("{}", message);
}
