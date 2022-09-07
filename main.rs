fn main() {
    /* let float_num: f32 = 3.14;
    let float_num_2: f64 = 3.2334327489;

    let tup: (i32, &str, u8) = (20, "Hello", 1);

    println!("{}", tup.1);

    let (a, b, c) = tup;
    println!("{}", a);

    let x = [1, 5, 6, 7];

    println!("{}", x[2]);

    let y = [2; 6]; // [2, 2, 2, 2, 2, 2]
    println!("{}", y[5]); */

    let a = 10;
    let b = a;
    let c = 15;
    let d = add(a, b);

    println!("A:{}", a);
    println!("B:{}", b);
    println!("C:{}", c);
    println!("D:{}", d);

    let message = String::from("Hello");
    let message2 = message;

    println!("{}", message);
    println!("{}", message2);
}

fn add(x: u32, y: u32) -> u32 {
    let sum = x + y;
    //return result;
    sum
}
