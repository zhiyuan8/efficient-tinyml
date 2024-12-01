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