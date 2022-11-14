fn main() {
    let a = 10;
    let b = &a;
    let c = &b;
    let d = &c;

    println!("{}", c);
    println!("{}", a == *b);
    println!("{}", a == **c);
    println!("{}", a == ***d);
}
