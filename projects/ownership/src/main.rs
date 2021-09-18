fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

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

    // ----- string slices -----
    let my_string = String::from("hello world");

    // both ok
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "hello world"; // string slices
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);
}
