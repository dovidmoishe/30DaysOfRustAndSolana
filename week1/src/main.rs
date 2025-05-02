fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len+= 1;
    }
    len
}
fn main() {
    struct User {
        name: String,
        age: i32,
    }
    let username = String::from("Dvaid");
    let david: User = User {name: username, age: 19};
    println!("{}", david.name);
    let sumoftwonumbers: i32 = sum(1, 2);
    for x in 1..5 {
        dbg!(x);
    }
    if sumoftwonumbers < 5 {
        println!("Sum is less than 5");
    }
    println!("Length: {}", collatz_length(11));
    println!("Hello, world! {sumoftwonumbers}");
}
