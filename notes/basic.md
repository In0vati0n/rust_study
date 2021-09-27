# Basic

ref: https://doc.rust-lang.org/stable/book/

## Table of Contents

- [Basic](#basic)
  - [Table of Contents](#table-of-contents)
  - [cargo command](#cargo-command)
  - [variables and mutability](#variables-and-mutability)
    - [const vs immutable variables](#const-vs-immutable-variables)
    - [shadow vs mutable variables](#shadow-vs-mutable-variables)
  - [Data Types](#data-types)
    - [Scalar Types](#scalar-types)
      - [Integer Types](#integer-types)
      - [Floating-Point Types](#floating-point-types)
      - [Boolean Type](#boolean-type)
      - [The Character Type](#the-character-type)
    - [Compound Types](#compound-types)
      - [The Tuple Type](#the-tuple-type)
      - [The Array Type](#the-array-type)
  - [Functions](#functions)
    - [Expression and Statement](#expression-and-statement)
    - [Functions with Return Values](#functions-with-return-values)
  - [Control Flow](#control-flow)
    - [`if` Expression](#if-expression)
      - [Using `if` in a let Statement](#using-if-in-a-let-statement)
    - [Loops](#loops)
      - [Returing Values from Loops](#returing-values-from-loops)
  - [Ownership](#ownership)
    - [Ownership](#ownership-1)
      - [Ownership Rules](#ownership-rules)
      - [Ways Variables and Data Interact: Move](#ways-variables-and-data-interact-move)
      - [Ways Variables and Data Interact: Clone](#ways-variables-and-data-interact-clone)
      - [Stack-Only Data: Copy](#stack-only-data-copy)
      - [Ownership and Functions](#ownership-and-functions)
      - [Return Values and Scope](#return-values-and-scope)
    - [References and Borrowing](#references-and-borrowing)
      - [Mutable References](#mutable-references)
      - [Dangling References](#dangling-references)
    - [Slice](#slice)
      - [String Slices](#string-slices)
      - [Other Slices](#other-slices)
  - [Structs](#structs)
    - [Defening and Instantiating Structs](#defening-and-instantiating-structs)
    - [Tuple Struct](#tuple-struct)
    - [Unit-Like structs without any fields](#unit-like-structs-without-any-fields)
    - [Method Syntax](#method-syntax)
    - [Assiciated Functions](#assiciated-functions)
  - [Enums and Pattern Matching](#enums-and-pattern-matching)
    - [Defining an Enum](#defining-an-enum)
    - [Defining an Enum with data](#defining-an-enum-with-data)
    - [The `match` Control flow operator](#the-match-control-flow-operator)
    - [Concise control flow with `if let`](#concise-control-flow-with-if-let)
  - [Managing Growing Projects with Packages, Crates and Modules](#managing-growing-projects-with-packages-crates-and-modules)
    - [Packages and Crates](#packages-and-crates)
    - [Modules](#modules)
    - [Paths](#paths)
    - [Privacy](#privacy)
    - [Making Structs and Enums Public](#making-structs-and-enums-public)
    - [`use` keyword](#use-keyword)
    - [Separating Modules into Different Files](#separating-modules-into-different-files)
  - [Common Collections](#common-collections)
    - [Vector](#vector)
    - [String](#string)
    - [HashMap](#hashmap)
  - [Error Handling](#error-handling)
    - [Unrecoverable Errors with `panic!`](#unrecoverable-errors-with-panic)
    - [Recoverable Errors with `Result`](#recoverable-errors-with-result)
    - [Propagating Errors](#propagating-errors)
    - [To `panic!` or Not to `panic!`](#to-panic-or-not-to-panic)
  - [Generic Types, Traits, and Lifetimes](#generic-types-traits-and-lifetimes)
    - [Generic Data Types](#generic-data-types)
    - [Traits: Defining Shared Behavior](#traits-defining-shared-behavior)
      - [Defining a Trait](#defining-a-trait)
      - [Implementing a Trait on a Type](#implementing-a-trait-on-a-type)
      - [Default Implementations](#default-implementations)
      - [Traits as Parameters](#traits-as-parameters)
      - [Specifying Multiple Trait Bounds with the `+` Syntax](#specifying-multiple-trait-bounds-with-the--syntax)
      - [Clearer Trait Bounds with `where` Clauses](#clearer-trait-bounds-with-where-clauses)
      - [Returning Types that Implement Traits](#returning-types-that-implement-traits)
      - [Using Trait Bounds to Conditionally Implement Methods](#using-trait-bounds-to-conditionally-implement-methods)
    - [Validating References with Lifetimes](#validating-references-with-lifetimes)
      - [Lifetime Annotation Syntax](#lifetime-annotation-syntax)
      - [Lifetime Annotations in Struct Definitions](#lifetime-annotations-in-struct-definitions)
      - [Lifetime Elision](#lifetime-elision)
      - [Lifetime Annotations in Method Definitions](#lifetime-annotations-in-method-definitions)
      - [The Static Lifetime](#the-static-lifetime)
  - [Writing Automated Tests](#writing-automated-tests)
    - [How to write tests](#how-to-write-tests)
      - [Checking results with the `assert!` macro](#checking-results-with-the-assert-macro)
      - [Testing equality with the `assert_eq!` and `assert_ne!` macros](#testing-equality-with-the-assert_eq-and-assert_ne-macros)
      - [Checking for panic with `should_panic`](#checking-for-panic-with-should_panic)
      - [Using `Result<T, E>` in tests](#using-resultt-e-in-tests)
    - [Controlling how tests are run](#controlling-how-tests-are-run)
    - [Test organization](#test-organization)
  - [Functional Language Features: Iterators and Closures](#functional-language-features-iterators-and-closures)
    - [Defining a closure](#defining-a-closure)

## cargo command

- `cargo build`: build a project
- `cargo run`: build and run a project in one step
- `cargo check`: build a project without producing a binary to check for errors
- `cargo build --release`: build for release version
- `cargo run --release`: build and run a project with release version

## variables and mutability

- default variables are **immutable**
- make variables mutable by adding `mut`

### const vs immutable variables
1. Don't allow to use `mut` with constants. Constants aren't just immutable by default -- they're always immutable.
2. Constants can be declared in **any scope**, including the global scope.
3. Constants may be set only to a constant expression.

*Rust's naming convention for constants is to use all **uppercase with underscores between words**, and underscores can be inserted in numeric literals to improve readability.*

```rust
const MAX_POINT: u32 = 100_000;
```

### shadow vs mutable variables
1. We'll get a **compile-error** if we accidentally try to reassign to the variable without using the `let` keyword.
2. By using `let`, we can perform a few transformations on a value but have the variable be **immutable** after those transformations have been completed.
3. We can change the type of the value but reuse the same name.

## Data Types
- Rust is a **statically typed** language.

### Scalar Types
- A **scalar** type represents a single value.
- Four primary scalar types: **integers**, **floating-point numbers**, **Booleans**, **characters**.

#### Integer Types

|Length|Signed|Unsigned|
|:---|:---|:---|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit(default)|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u128|
|arch|isize|usize|

*Signed numbers are stored using two's complement representation*

Integer Literals in Rust:

|Number literals|Example|
|:---|:---|
|Decimal|98_222|
|Hex|0xff|
|Octal|0o77|
|Binary|0b1111_0000|
|Byte(u8 only)|b'A'|

#### Floating-Point Types

- `f32`
- `f64`(default)

#### Boolean Type

- `true`
- `false`

```rust
let t = true;
let f: bool = false;
```

#### The Character Type

- Use **single quotes**.
- **Four bytes** in size and represents a Unicode Scalar Value.

### Compound Types

#### The Tuple Type

- Group together a number of values with a **variety of types** into one compound type.
- Have a **fixed length**: once declared, they cannot grow or shrink in size.

```rust
// create a tuple
let tup: (i32, f64, u8) = (500, 6.4, 1);

// destructuring
let (x, y, z) = tup; 

// access with index
let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;
```

#### The Array Type

- Every element of an array must have the **same type**.
- Fixed length like tuples.
- Raise a *runtime* error at the point of using an invalid value in the indexing operation.

```rust
let a = [1, 2, 3, 4, 5];
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // ==> let a = [3, 3, 3, 3, 3];

let first = a[0];
let second = a[1];
```

## Functions

- Rust code uses *snake* case as the conventional style for function and variable names.
- Rust doesn't care about where you define your functions, only that they're defined somewhere.
- **Must** declare the type of each parameter.

### Expression and Statement

- Rust is an **expression-based** language.
- **Statements** are instructions that perform some action and do not return a value.
- **Expressions** evaluate to a resulting value.
- Calling a function is an expression. Calling a macro is an expression. The block that we use to create new scopes, `{}`, is an expression.

    ```rust
    fn main() {
        let x = 5;

        let y = {
            let x = 3;
            x + 1 // without a semicolon
        };
    }
    ```

### Functions with Return Values

- In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
- Use `return` keyword and specifying s value to return early from a expression.

    ```rust
    fn five() {
        5
    }
    ```

## Control Flow

### `if` Expression

- Condition must be a bool. If the condition isn't a bool, we'll get an error.
- RUst will not automatically try to convert non-Boolean types to a Boolean.

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition wae true");
    } else {
        println!("condition wae false");
    }
}
```

#### Using `if` in a let Statement

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

### Loops

- `loop`

    ```rust
    loop {
        println!("again!");
    }
    ```

- `while`

    ```rust
    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    ```

- `for`

    ```rust
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    ```
    
    *Faster than `while` version, becaure it won't check index every iteration.*
    
    ```rust
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    ```

#### Returing Values from Loops

```rust
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

## Ownership

### Ownership

#### Ownership Rules

- Each value in Rust has a variable that's called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

- When variable goes out of scop, Rust calls a special function for us, called `drop`.

#### Ways Variables and Data Interact: Move

- copy only stack memory
- Rust will never automatically create "deep" copies of your data.

```rust
let s1 = String::from("hello");
let s2 = s1; // move

// println!("{}", s1); // error! s1 is no longer valid
println!("{}", s2); // ok
```

#### Ways Variables and Data Interact: Clone

- copy both stack and heap memory

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // clone

println!("s1 = {}, s2 = {}", s1, s2);
```

#### Stack-Only Data: Copy

- Rust won't let us annotate a type with the **Copy trait** if the type, or any of its parts, has implemented the **Drop trait**.
- Here are some of the types that implement Copy:

    - All the integer types, such as u32.
    - The Boolean type, bool, with values true and false.
    - All the floating point types, such as f64.
    - The character type, char.
    - Tuples, if they **only** contain types that also implement Copy.
    
#### Ownership and Functions

- Passing a variable to a function will move or copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello"); // s comes in scope

    takes_ownership(s); // s's value moves into the function
                        // and so is no longer valid here

    let x = 5;          // x comes into scope

    makes_copy(x);      // x would move into the function
                        // but i32 is Copy, so it's okay to still use x afterward
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_coy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goe out of scope. Nothing special happens.
```

#### Return Values and Scope

- The ownership of a variable follows the same pattern everty time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

```rust
fn main() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    
    let s2 = String::from("hello"); // s2 comes into scope
    
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back,
                                       // which also moves its return value into s3 
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved,
// so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String { // gives_ownership will move its return value into
                                 // the function that calls it
    
    let some_string = String::from("hello"); // some_string come into scope
    
    some_string // some_string is returned and moves out to the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
```
### References and Borrowing

- Reference parameter will not be dropped when it goes out of scope, because we don't have ownership.
- Call having references as function parameters borrowing.
- Just as variables are immutable by default, so are references. We're not allowed to modify something we have a reference to.

```rust
fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);
    
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

#### Mutable References

- You can have **only one** mutable reference to a particular piecee of data in a particular scope. (prevent data races at compile time)

```rust
fn main() {
    let mut s = String::from("hello");
    
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- We can use curly brackets to create a new scope, allowing for multiple mutable references.

```rust
fn main() {
    let mut s = String::from("hello");
    
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    
    let r2 = &mut s;
}
```

- We also cannot have a mutable reference while we have an immutable one.
- Multiple immutable references are okay.
- A reference's scope starts from where it is introduced and continues through the last time that references is used.

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not used after this point
    
    let r3 = &mut s; // no problem
    println!("{}", r3);
    
    // println!("{}", r1); // error!
    // mutable reference cannot be used while have an immutable 
    // reference one
}
```

#### Dangling References

- Compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

### Slice

- Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

#### String Slices

- A **string slice** is a reference to part of a String.

```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5]; // ==> let hello = &s[..5];
    let world = &s[6..11]; // ==> let world = &s[6..];
    let whole_string = &s[...];
}
```

- String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

#### Other Slices

- There's a more general slice type.

```rust
#![allow(unused)]
fn main() {
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3]; // type is &[i32]

assert_eq!(slice, &[2, 3]);
}

```

## Structs

### Defening and Instantiating Structs

- Basic define and use.

    ```rust
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    fn main() {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
    }
    ```
    
- Using the Field Init Shorthand when variables and Fields have the same name.

    ```rust
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    ```
    
- Creating instances from other instances with struct update syntax.

    ```rust
    fn main() {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
    
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1
        };
    }
    ```

### Tuple Struct

- Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.

```rust
fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Unit-Like structs without any fields

- These are called unit-like structs because they behave similarly to (), the unit type.

### Method Syntax

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### Assiciated Functions

Another useful feature of impl blocks is that we’re allowed to define functions within impl blocks that don’t take self as a parameter.


## Enums and Pattern Matching

### Defining an Enum

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {}
```

### Defining an Enum with data

```rust
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
```

- There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.

```rust
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```

- Option Enum

```rust
enum Option<T> {
    None,
    Some(T),
}
```

### The `match` Control flow operator

- Patterns an be made up of **literal values**, **variable names**, **wildcards**, and many other things.

- Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- The `_` pattern will match any value.

### Concise control flow with `if let`

```rust
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("other"),
    }
}

fn main() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("other");
    }
}
```

## Managing Growing Projects with Packages, Crates and Modules

- **Module System**:

    - **Packages**: A Cargo feature that lets you build, test, and share crates
    - **Crates**: A tree of modules that produces a library or executable
    - **Modules** and **use**: Let you control the organization, scope and privacy of paths
    - **Paths**: A way of naming an item, such as a struct, function or module

### Packages and Crates

- A **crate** is a binary or library.

    - The *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate.
    
- A **package** is one or more crates that provide a set of functionality.

    - A package contains a *Cargo.toml* file that describes how to build those crates.
    
    - A package can contain at most one library crate. It can contain as many library crates as you'd like, but it must contain as least one crate (either library or binary).
    
    - *src/main.rs*: the crate root of a binary crate with the same name as the package.

    - *src/lib.rs*: the crate root of a library crate with the same name as the package.
    
    
    - A package can have multiple binary crates by placing files in the *src/bin* directory: each file will be a separate binary crate.
    
### Modules

- **Modules** 

    1. let us organize code within a crate into groups for readability and easy reuse.
    2. control the *privacy* of items, which whether an item can be used by outside code(*public*) or is an internal implementation detail and not available for outside use(*private*).
    
    
- Define a module by starting with the `mod` keyword

### Paths

- A **Path** can take two forms:

    - An *absolute path* starts from a crate root by using a crate name or a literal `crate`.
    - A *relative path* starts from the current module and uses `self`, `super` or an identifier in the current module.
    
### Privacy

- All items (functions, methods, structs, enums, modules, and constants) are **private** by default. Items in a parent module can't use the private items inside child modules, but items in child modules can use the items in their ancestor modules.

### Making Structs and Enums Public

- If we use `pub` before a struct definition, we make the struct public, but the struct's fields will still be private. We can make each field public or not on a case-by-case basis.

- If we make an enum public, all of its variants are then public.

### `use` keyword

- We can bring a path into a scope once and then call the items in that path as if they're local items with the `use` keyword.

- You can also bring an item into scope with `use` and a relative path.

- Providing new names with the `as` keyword.

- Re-exporting names with `pub use`

- Using nested paths to clean up large `use` lists: `use std::io::{self, Write};`.

- The glob operator: `use std::collections::*;`

### Separating Modules into Different Files

- Using a semicolon after `mod abc` rather than using a block tells Rust to load the contents of the module from another file with the same name as the module.

## Common Collections

- Unlike the built-in array and tuple types, the data these collections point to is stored on the **heap**, which means the amount of data dose not need to be known at compile time and can grow or shrink as the program runs.

- Common collections

    - A **vector** allows you to store a variable number of values next to each other.
    - A **string** is a collection of characters.
    - A **hash map** allows you to associate a value with a particular key.
    
### Vector

- Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.

- Vectors can only store values of the same type.

    ```rust
    // ----- create a vector -----

    let v: Vec<i32> = Vec:new();

    let v = vec![1, 2, 3];

    // ----- updating a vector -----

    v.push(5);
    v.push(6);

    // ----- drop -----

    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v gose out of scope and is freed here

    // ----- get elements -----

    // index start at 0
    let third: &i32 = &v[2];

    match v.get(2) { // give Option<&i32>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // ----- iterating over the values in a vector -----

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // ----- using an enum to store multiple types -----

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    ```

### String

- Both `String` and string slices are UTF-8 encoded.

    ```rust
    // ----- create a new string -----

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // ----- updating a string -----

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String:from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('l')

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // + operator uses the add method

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // doesn't take ownership of any of its parameters

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    ```

### HashMap

- Hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

    ```rust
    use std::collections::HashMap;

    // ----- normal create -----

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // ----- create with iterators -----

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // ----- ownership -----

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // ----- accessing values -----

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // ----- iter ------

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // ----- updating value -----

    // 1. overwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // {"Blue": 25}

    // 2. only inserting a value if the key has no value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // 3. updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    ```

## Error Handling

### Unrecoverable Errors with `panic!`

- Can set `RUST_BACKTRACE` to 1 to get a backtrace of excatly what happened to cause the error. `RUST_BACKTRACE=1 cargo run`.

```rust
fn main() {
    panic!("crash and burn");
}
```

### Recoverable Errors with `Result`

- Like the `Option` enum, the `Result` enum and its variants have been brought into scope by the prelude.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `unwrap`, return value inside the `Ok` while the `Result` value is the `Ok` variant, ohterwise `unwrap` will call the `panic!` macro. 

    ```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").unwrap();
    }
    ```

- `expect` is similar to `unwrap`, let us also choose the `panic!` error message.

    ```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
    ```

### Propagating Errors

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

use `?` operator:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::Open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

use chain:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

**The `?` Operator can be used in functions that return `Result`**


### To `panic!` or Not to `panic!`

- ref: https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html

## Generic Types, Traits, and Lifetimes

### Generic Data Types

- In function

    ```rust
    fn largest<T>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    ```

- In struct definition

    ```rust
    struct Point<T, U> {
        x: T,
        y: U,
    }
    ```

- In enum definition

    ```rust
    enum Option<T> {
        Some(T),
        None,
    }
    ```

- In method definition

    ```rust
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    ```

    Generic type parameters in a struct definition aren’t always the same as those you use in that struct’s method signatures.

    ```rust
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
    ```

- **Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. So we pay no runtime cost for using generics.**

### Traits: Defining Shared Behavior

Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

#### Defining a Trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

#### Implementing a Trait on a Type

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

**One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to our crate.**

#### Default Implementations

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}
```

#### Traits as Parameters

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

equivalent to:

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

#### Specifying Multiple Trait Bounds with the `+` Syntax

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

equivalent to:

```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

#### Clearer Trait Bounds with `where` Clauses

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

equivalent to:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

#### Returning Types that Implement Traits

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

#### Using Trait Bounds to Conditionally Implement Methods

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

### Validating References with Lifetimes

- Every reference in Rust has a *lifetime*.

#### Lifetime Annotation Syntax

- Lifetime annotations don't change how long any of the references lives. 

- Lifetime annotations describe the relationships of the Lifetimes of multiple references to each other without affecting the Lifetimes.

- Syntax: start with an apostrophe(`'`) and are usually all lowercase and very short.

    ```rust
    &i32            // a reference
    &'a u32         // a reference with an explicit lifetime
    &'a mut i32     // a mutable reference with an explicit lifetime
    ```


#### Lifetime Annotations in Struct Definitions

```rust
// This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

#### Lifetime Elision

- Rules:

    1. each parameter that is a reference gets its own lifetime parameter
    2.  if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    3.  if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters

#### Lifetime Annotations in Method Definitions

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

#### The Static Lifetime

One special lifetime we need to discuss is `'static`, which means that this reference can live for the entire duration of the program.

## Writing Automated Tests

### How to write tests

The bodies of test functions typically perform these three actions:

1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.

Add `#[test]` on the line before `fn` to change a function into a test function.

#### Checking results with the `assert!` macro

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

#### Testing equality with the `assert_eq!` and `assert_ne!` macros

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

#### Checking for panic with `should_panic`

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

#### Using `Result<T, E>` in tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```


### Controlling how tests are run

- use `--test-threads` to control the number of threads to use.
- use `--show-output` to tell Rust to also show the output of successful tests at the end
- use `cargo test testname` to run a specific test
- use `cargo test namepattern` to run a subset of tests
- use `#[ignore]` to make the test ignored, use `cargo test -- --ignored` to run only the ignored tests

### Test organization

- Unit Tests: small and more focused, testing one module in isolation at a time, and can test private interfaces
- Integration Tests: entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test

## Functional Language Features: Iterators and Closures


### Closure
#### Defining a closure

- Closures don't require you to annotate the types of the parameters or the return value.

```rust
let a_closure = |num| {
    num
};

let a_closure_with_type = |num: u32| -> u32 {
    num
};

let a_closure_without_brackets = |num| num;
```

#### Storing Closures Using Generic Parameters and the `Fn` Traits

```rust
struct Cacher<T>
where
    T : Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```

#### Capturing the Environment with Closures

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

- Closures can capture values from their environment in three ways:

    1. take ownership: `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure's environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The `Once` part of the name represents the fact that the closure can't take ownership of the same variables more than once, so it can be called only once.
    2. borrow mutably: `FnMut` can change the environment because it mutably borrows values.
    3. borrow immediately: `Fn` borrows values from the environment immutably.
    
- When you create a closure, Rust infers which trait to use based on how the closure uses the values from the environment.

### Iterators
#### Processing a Series of Items with Iterators

```rust
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}
```

- All iterators implement a trait named `Iterator` that is defined in the standard library.

    ```rust
    pub trait Iterator {
        type Item;
        
        fn next(&mut self) -> Option<Self::Item>;
        
        // methods with default implementations elided
    }
    ```

- The `Iterator` trait only requires implementors to define one method: the `next` method, which returns one item of the iterator at a time wrapped in `Some` and, when iteration is over, returns `None`.

#### Methods that Consume the iterator

- Methods that call `next` are called *consuming adaptors*, like `sum`.

- You can find out about these methods by looking in the standard library API documentation for the `Iterator` trait.

#### Methods that Produce Other Iterators

- *Iterator adaptors* allow you to change iterators into different kinds of iterators.

#### Creating Our Own Iterator with the `Iterator` Trait

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```
