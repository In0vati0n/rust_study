fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = "hello world";
    let b = String::from("nihao shijie");

    let ret = longest(a, b.as_str());
    println!("The longest str: {}", ret);
}
