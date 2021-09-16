fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = &s1;
    println!("{}, world!", s2);

    let hello_world = String::from("hello world");
    let hello = &hello_world[..5];
    let world = &hello_world[6..];
    println!("1 {} 2 {}", hello, world);
}
