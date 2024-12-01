# Rust Tutorial

## Rust Basics

### Installation
- [Download Rust](https://www.rust-lang.org/tools/install)

### Using Cargo
`cargo` is used to create, compile, and run Rust programs.

```bash
# Create a new Rust project
cargo new <projectname>

# Clean the project
cargo clean

# Compile the project
cargo build

# Run the compiled project
cargo run
```

### Rustup
`rustup` is a toolchain installer for Rust, allowing you to manage Rust versions and associated tools.

## Crate

A crate is a package of Rust code. It can be a binary or a library.

### Rust Standard Library
- The standard library is `std`.
    - Documentation: [doc.rust-lang.org/std/](https://doc.rust-lang.org/std/)

### External Crates
- Install external crates using:
  ```bash
  cargo add <crate_name>
  ```
- Find crates at [crates.io](https://crates.io/)
- Documentation for crates is available at [docs.rs](https://docs.rs/)

### Importing Crates
Use the `use` keyword to import a crate.

```rust
use rand::{seq::SliceRandom, thread_rng};
```

## Important Concepts

### Arrays vs Vectors
- **Arrays** have a fixed size.
- **Vectors** can grow and shrink dynamically.

Example:
```rust
let array = [1, 2, 3]; // Fixed size
let mut vector = vec![1, 2, 3]; // Dynamic size
vector.push(4); // Now vector is [1, 2, 3, 4]
```

### Mutability
In Rust, variables are **immutable** by default. Use `mut` to make them mutable.

```rust
// Immutable
let numbers = vec![1, 2, 3];
// numbers.push(4); // This will cause an error

// Mutable
let mut numbers = vec![1, 2, 3];
numbers.push(4); // This is allowed
```

### Implicit Return
Rust functions return the last expression implicitly, without needing the `return` keyword.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // No need for `return`
}
```

### Attributes
Attributes provide metadata about the code. For example, `#[derive(Debug)]` allows you to print debug information.

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 5, y: 10 };
    println!("{:?}", point); // Prints: Point { x: 5, y: 10 }
}
```

### Structs
A `struct` defines a custom data type.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

### Inherent Implementation: Method vs Associated Function

- **Associated Functions**: Tied to the struct, don't have `self` as a parameter. Use the `::` operator.
  ```rust
  impl Rectangle {
      fn new(width: u32, height: u32) -> Rectangle {
          Rectangle { width, height }
      }
  }

  let rect = Rectangle::new(30, 50);
  ```

- **Methods**: Tied to a specific instance of a struct, have `self` as a parameter. Use the `.` operator.
  ```rust
  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }

  let rect = Rectangle { width: 30, height: 50 };
  println!("Area: {}", rect.area());
  ```

# Rust Memory System : Ownership, Borrowing, and Lifetimes

## Ownership

Ownership is a set of rules that governs how a Rust program manages memory. The key principles of ownership are:

1. **Each value in Rust has a variable that's called its owner.**
2. **There can only be one owner at a time.**
3. **When the owner goes out of scope, the value will be dropped.**

Example:
```rust
{
    let s = String::from("hello"); // s is the owner of the String
    // s can be used here
} // s goes out of scope and the String is dropped
```

## Borrowing

Borrowing allows you to have references to data without taking ownership. There are two types of borrowing:

- **Immutable Borrowing**: Allows multiple read-only references.
- **Mutable Borrowing**: Allows a single read-write reference.

Example:
```rust
fn main() {
    let s = String::from("hello");

    // Immutable borrow
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    // Mutable borrow
    let mut s = String::from("hello");
    change(&mut s);
    println!("Changed string: {}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

## Lifetimes

Lifetimes are a way of describing the scope for which a reference is valid. They ensure that references do not outlive the data they point to. In C++, you might have a pointer to a variable, and if that variable goes out of scope, the pointer becomes a dangling pointer. Rust's lifetimes ensure that this situation doesn't happen by checking at compile time that all references are valid.

Example:
```rust
fn main() {
    let string1 = String::from("hello");
    let string2 = String::from("world");

    let result = longest(&string1, &string2);
    println!("The longest string is '{}'", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Function longest: This function takes two string slices (&str) and returns the longest one. The 'a is a lifetime parameter that tells Rust that the returned reference will be valid as long as both input references are valid.

### Key Points

- **Ownership** ensures memory safety without a garbage collector.
- **Borrowing** allows you to use data without taking ownership.
- **Lifetimes** prevent dangling references by ensuring references are valid.

These concepts are fundamental to Rust's memory safety guarantees and help prevent common bugs such as null pointer dereferencing and data races.


# Modules

Three ways to define modules in Rust:

1. **Create a `mod` in an existing file.**

   You can define a module directly within an existing file using the `mod` keyword. Functions, structs, enums, etc., must have `pub` to be accessed outside the module.

   ```rust
   mod my_module {
       pub fn say_hello() {
           println!("Hello from my_module!");
       }
   }

   fn main() {
       my_module::say_hello();
   }
   ```

2. **Create a module in a new single file in the same folder.**

   You can create a new file named `my_module.rs` in the same directory as your main file. In `my_module.rs`, define your module:

   ```rust
   // my_module.rs
   pub fn say_hello() {
       println!("Hello from my_module!");
   }
   ```

   Then, in your main file, include the module:

   ```rust
   // main.rs
   mod my_module;

   fn main() {
       my_module::say_hello();
   }
   ```

3. **Split a module into multiple files.**

   You can organize a module into multiple files by creating a directory with the module's name and placing a `mod.rs` file inside it. For example, create a directory `my_module` with a `mod.rs` file:

   ```rust
   // my_module/mod.rs
   pub mod greetings;

   // my_module/greetings.rs
   pub fn say_hello() {
       println!("Hello from greetings!");
   }
   ```

   In your main file, include the module:

   ```rust
   // main.rs
   mod my_module;

   fn main() {
       my_module::greetings::say_hello();
   }
   ```