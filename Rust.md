# Motivation
- safe, easy to use parallel computing paradigms, fast C and C++ have hundreds of rules for avoiding undefined behavior
- mostly common sense items like don’t access memory you shouldn’t, don’t let arithmetic operations overflow, don’t divide by zerobut the **compiler does not enforce these rules**.  Indeed, the following program compiles without errors or warnings even though it could be used (e.g. 1988 Morris Worm) to access other programs on the call stack

```c
int main(int argc, char **argv) {
	unsigned long a[1];
	a[3] = 0x7ffff7b36cebUL;
	return 0;
}
```

With Rust, if your program passes the compiler’s checks, it is free of undefined behavior: dangling pointers, double-frees, and null pointer dereferences are caught at compile time, while array references are secured with a mix of compile-time and run-time checks.

# Hello World
- `cargo new <project_name>` to create a new project
  - creates a `Cargo.toml` file to define the project structure, dependencies, etc. 
- Add your logic to the `main.rs` file 
  - either a `src/lib.rs`, `src/main.rs`, a [lib] section, or [[bin]] section must be present
  - the `main` function is treated as the entry point for executable projects

# Syntax
- **#[X]** : An `attribute`, the standard way of annotating/decorating functions and declarations
- **x.f() vs y::f()** : value member/method access vs type/module/namespace member access
  - `x` is a value/object (i.e. has state), whereas `y` is abstract (i.e. no `self` or state)
- **&x** : get a reference to (but not ownership of) the elements of `x`
- ***x** : dereference to read the value
- **x!** : invoke a macro (vs a function)
- **[1..]** : sequence slicing
  - [x..] (x onwards, inclusive), 
- **..<struct>** : `struct` update syntax, functions like a dict/map merge
- **?** : shorthand for the common pattern of returning the data on success (`OK`) or an error on error (`Err`).
```rust
let output = match File::create(filename) { 
    Ok(f) => f,
    Err(e) => { return Err(e); }
};
// the above can be simplified to
let output = File::create(filename)?;
``` 
- **x** : xxx
- **x** : xxx
- **x** : xxx

## Imports
- `use actix_web::{HttpServer};`
  - import module and create a top-level alias for it
- `use std::str::FromStr;`
  - import a trait from a module
- `use std::env;`
  - import a module

# Types
| type | code | values | description |
| :----: | :---: | :-------: | :-------: |
| Integer     |  i8,i16,i31,i64,i128 (or uX) | `12` | |
| Float     |  f32, f64    |   `3.333`      | |
| Boolean       | bool   | `true, false`        | |
| Character      |   char    | `'*', '\t'`  | |
| String     |   ""    | `String::from("bob is silly")`  | dynamically sized, UTF-8 |
| Slice     |  &str, &[i32] | `"a literal string"`, `[1,2,3,4][1..2]` | |
| Vector     | Vec<T>  | `vec![0.32, 3.3, 3.2]` | |
| Enumeration     |   `enum Attend { OnTime, Late (u32) }`   | `Attend::Late(5)`,`Attend::OnTime`  | |
| Structure  |  `struct S { x: f32, y: f32 }`   | `S { x: 120.0, y: 209.0 }` | |
| Tuple  |   `struct T (i32, char);`    | `T(120, 'X')` | |
| Closure   |   `\|\| {}`  | `\|a, b\| { a*a + b*b }` | |
| unit (void) | `()` | `()`| |
| Option<T> | `Option<f32>` | `Some("Dr."), None`| either None (absent) or Some(v)|
| Result<T, Error> | `Result<u16, ParseIntError>` | `Result<u64, ParseIntError>`| Result of operation that may fail: either Ok(v) or Err(e) |


## Tuples
Fixed length, heterogenous sequence
```rust
let my_tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = my_tup;      // destructure
let five_hundred = my_tup.0; // access tuples elements by index
```
## Arrays
Arrays are fixed-length, homogeneous sequences
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];  // i32 type, 5 elements
let a = [3; 5];                     // 5 instances of the value 3
let first = a[0]; // access array elements by index
```

## Functions
```rust
fn plus_one(x: i32) -> i32 {
    x + 1;  // compile error, the ; makes this a statement (vs expression), so fn has NO return value
}
```

# Control Flow
## Conditionals
```rust
if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}

let num = if number%3==0 {3} else {2};
```

## Iteration
### Loop
The `loop` expression can be used to iterate until an explicit `break` is called.  Instances of `loop` can be assigned names such that `break` statements can specify which `loop` is being exited.
```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;  // assigns counter * 2 to result
    }
};

println!("The result is {result}");
```

### For and While
```rust
let a = [10, 20, 30, 40, 50];

for element in a {  // iterate over elements
    println!("the value is: {element}");
}

for i in (1..4).rev(){ // print a range in reverse order
    println!("{i}");
}

let mut index = 0;
while a[index] < 40 {
    println!("{i}");
    index += 1;
}
```

# Concepts
## Refresher: The Stack and the Heap
From: https://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap

>The stack is the memory set aside as scratch space for a thread of execution. When a function is called, a block is reserved on the top of the stack for local variables and some bookkeeping data. When that function returns, the block becomes unused and can be used the next time a function is called. The stack is always reserved in a LIFO (last in first out) order; the most recently reserved block is always the next block to be freed. This makes it really simple to keep track of the stack; freeing a block from the stack is nothing more than adjusting one pointer.

>The heap is memory set aside for dynamic allocation. Unlike the stack, there's no enforced pattern to the allocation and deallocation of blocks from the heap; you can allocate a block at any time and free it at any time. This makes it much more complex to keep track of which parts of the heap are allocated or free at any given time; there are many custom heap allocators available to tune heap performance for different usage patterns.

>The stack is faster because the access pattern makes it trivial to allocate and deallocate memory from it (a pointer/integer is simply incremented or decremented), while the heap has much more complex bookkeeping involved in an allocation or deallocation. Also, each byte in the stack tends to be reused very frequently which means it tends to be mapped to the processor's cache, making it very fast. Another performance hit for the heap is that the heap, being mostly a global resource, typically has to be multi-threading safe, i.e. each allocation and deallocation needs to be - typically - synchronized with "all" other heap accesses in the program.

## Ownership
Operating systems allocate a stack and a heap for memory management during thread execution.  Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often, but knowing that **the main purpose of ownership is to manage heap data** can help explain why it works the way it does.

Other languages have notions of deep copies (e.g. `clone`) vs shallow (e.g. by reference) copies, but with Rust it is better to think of object passing in terms of a `move`.  Ownership of a value is transfered/moved to a new reference and as a result old references to that value are invalidated (and will result in compilation errors if referenced).  This is the default behavior, although with simple types (e.g. numbers and other classes that have the `Copy` `trait`) with fixed sizes that are known at compile time, space is allocated on the stack (e.g. deep copy behavior) for them and ownership does not apply.

```rust
let x = 5;
some_function(x);  // i32 implements Copy trait, so ownership is meaningless
```

### Ownership of variable-sized types
```rust
let s1 = String::from("hello");
let s2 = s1;
// s1 is no longer valid, any references to it will c_error
```
Ownership of a value is moved to the functions scope if the value is simply passed into it.  Ownership of a function's return value is moved to the caller
```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // any function call takes ownership
    // at this point, s is no longer valid

    let s1 = String::from("hey again");
    let s2 = borrow(s1) // s1 is now invalid

} // At this point, s* go out of scope

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // At this point, some_string goes out of scope and is dropped

fn borrow(s: String) {
	s    // ownership of return value is given to caller
}
```

### References
- At any given time, a value can have either one mutable reference or any number of immutable references.
- References must always be valid.

```rust
fn main() {
    let s = String::from("hello");
    change(&s);  // pass a (read-only) reference to a value
}

// there is no transfer of ownership, wrt s, in this function call
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but since scope does not own s, s is not dropped
```

#### Mutable references
The typical use for this pattern (in contrast to returning ownership of the argument) would be in functions where it's unclear at compile time how much memory needs to be allocated to hold the result and/or it might be a good idea to reuse previously allocated memory, and therefore these *decisions are left to the caller*?
```rust
fn main() {
	let mut s = String::from("hello");
	// for a given scope, there can be only 1 mutable reference to any value
	ew_ah_push_it(4, &mut s)
}

//mutable (read/write) reference
fn ew_ah_push_it(s: &mut String) {
	s.push_str(" world!")
}
```


## Structs
```rust
struct User {   // define struct
    active: bool,
    email: String, // use String (vs &str) so that Struct owns all of its contents ... see lifetimes concept
    sign_in_count: u64,
}

fn main() {
    let user1 = User {  // instantiate struct
        active: true,
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    print!("{}", user1.active); // dot notation to access fields
    // user1.active = false;  // C_ERROR, would work if `let mut user1 ...`
    
    let user2 = User {
        active:false,
        ..user1           // struct update ... use values from user1 for all fields not explicitly defined
    };

    print!("{}", user1.active);  // fine, active has Copy trait
    print!("{}", user1.email);  // c_error, ownership moved to user2.
    //If new email had been specified for user2, user1 would still be usable as a whole
    print!("{}", user1.active);  // fine, active has Copy trait
    print!("{}", user1.email);  // C_ERROR, ownership moved to user2
    print!("{}", user1);  // C_ERROR, the trait `std::fmt::Display` is not implemented for `User`
    println!("user1 is {user1:?}");  // C_ERROR,  :? -> print Debug format
    println!("user1 is {user1:#?}");  // C_ERROR,  #:? -> pretty print Debug format
    dbg!(&user1)    // print file, line number, and Debug format to stderr
}
```

### Printing structs


## Attributes
The #[test] marker is an example of an attribute. Attributes are an open-ended system for marking functions and other declarations with extra information, like attributes in C++ and C#, or annotations in Java. They’re used to control compiler warnings and code style checks, include code conditionally

### Examples
```rust
#[derive(Deserialize)]
struct GcdParameters { 
    n: u64,
    m: u64,
}
```
Tells the `serde` crate to examine the type when the program is compiled and automatically generate code to parse a value of this type from data in any of the std formats supported by serde (e.g. JSON, YAML, TOML)

## Traits
`use std::str::FromStr;`
A *trait* is a collection of methods that types can implement (e.g. an interface). Any type that implements the FromStr trait has a from_str method that tries to parse a value of that type from a string.

# Core classes
## Result
Functions that might fail (e.g. IO or OS calls) can return `Result` types.  Results can be either:
- **Ok** : results are resulted
- **Err** : error code returned

The `expect()` method can be invoked on a `Result` to branch based on success or error:
- **Ok** : return function results
- **Err** : print and exit the program

Rust does not have exceptions: all errors are handled using either `Result` or panic

## Option
```rust
enum Option<T> {
    None,
    Some(T),
}
```
Used in concert with the `match` function to distinctly handle scenarios where nothing (None) is returned vs an actual result (Some).

# IO
## JSON
[**Dependency**](https://github.com/serde-rs/json): serde_json = "1.0"

serde_json::Value struct
```rust
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

# Gotchas
- `!` marks a macro invocation, not a function call
- Rust does not have exceptions: all errors are handled using either a `Result` or a panic
# Tooling

## Cargo
Cargo is Rust’s compilation manager, package manager, and general-purpose tool. You can use Cargo to start a new project, build and run your program, and manage any external libraries your code depends on. 

### Usage
- `cargo new hello --bin` – create a new (binary executable) project
- `cargo build` - compile your code
  - add `--release` for optimization (slower compilation, faster execution)
- `cargo run` – compile and run the resulting executable
- `cargo clean` – remove compiled files and the `target` folder
- `cargo test` - run all functions marked with the `test` attribute

# Glossary
- **crate** : A packaged library or executable
- **rustc** : Rust compiler, often 
- **rustdoc** : Rust documentation tool. Doc comments, denoted by `///`, support Markdown and if present are automatically turned into useful html documentation for your code
- **rustup** : tool for managing Rust installations
- **cargo** : dependency management tool for Rust (e.g. Maven or Pip)


# Sources
1. https://doc.rust-lang.org/book/title-page.html <a name="source_1"></a>
2. EVCXR - repl https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md