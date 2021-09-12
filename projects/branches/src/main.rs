fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("condition was false");
    }

    let cond = true;
    let number = if cond { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
