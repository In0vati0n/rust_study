# Basic

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
- Rust is a *statically typed* language.

### Scalar Types
- A *scalar* type represents a single value.
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
- *Statements* are instructions that perform some action and do not return a value.
- *Expressions* evaluate to a resulting value.
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
    
    *Faster than `while` version, beacaure it won't check index every iteration.*
    
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

