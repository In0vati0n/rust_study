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