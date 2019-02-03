<!-- $theme: gaia -->

# Introduction to @color[orange](**Rust**)
#### by [Tim Bess](https://github.com/tdbgamer)


---

## What is Rust?
- @color[orange](**Non-GC**)'d systems level language.
- Focused on @color[orange](**safety**), @color[orange](**performance**), @color[orange](**concurrency**), and @color[orange](**interoperability**).
- Designed to make systems level software more accessible to developers.

---

## Why do I need Rust?
- High level languages offer low mental overhead and speed of development, but aren't usually performant.
    Developers can drop down into Rust where traditionally C had been used.
- Applications with high cost of failure or where correctness and refactorability are valuable.

---

## Rust Features

- Will not segfault.
- Will not allow data races.
- Will not leak memory.
- Zero cost abstractions.

---

## Safe references
- Pointer aliasing and mutability are not mixed (compile-time guarantee).
- Pointers never live longer than the data they hold.
- Array types provide safe bounds checking, unlike C.

---

## Data races
- Rust guarantees pointers must be exclusively aliased or mutated.
- Send/Sync traits force types to explicitly define whether they can be sent between threads or shared between threads respectively

---

### Ownership System
#### Ownership Rules
- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

#### Borrowing Rules
- At any given time, you can have _either_ one mutable reference or any number of immutable references.
- References must always be valid.

+++

## Ownership System

#### Example 1
```rust
fn main() {
    {
        // foo is owned in this scope
        let foo = String::from("bar");
    } // foo is dropped and freed
    
    // Fails to compile
    println!("{}", foo);
}
```

+++

## Ownership System

#### Example 2
```rust
// foo is an owned string
fn takes_string(foo: String) {
    // Does things with foo
} // Drops foo

fn main() {
    let foo = String::from("bar");

    // Ownership is MOVED to takes_string
    takes_string(foo);
    
    // Fails to compile
    // string was moved into takes_string
    println!("{}", foo);
}
```

+++

## Ownership System

#### Example 3
```rust
// foo is an immutably borrowed string
fn takes_string(foo: &String) {
    // Does things with foo
} // drops borrow

fn main() {
    let foo = String::from("bar");

    // give an immutable borrow to takes_string
    takes_string(&foo);
    
    // Compiles successfully!
    println!("{}", foo);
} // foo is dropped
```

---



---

## Usecase: Python

#### Pros
- Approachable for beginners.
- Excellent for fast prototyping while requirements are poorly defined.
- Allow for clean, terse code.
- Plethora of libraries at your fingertips.

+++

## Usecase: Python

#### Cons
- Very slow
    - pypy is making things better, but still needs work to be compatible with many Python libraries.
    - Scientific Python users have to offload as much work as possible onto pandas/numpy to be performant.
- Difficult to refactor.
    - Efforts like Mypy might improve this in the future, but would require mass adoption to be useful in many cases.

+++

## Oxidizing Python

#### CFFI Method
- CBindgen + Milksnake allow for relatively seamless build process for calling out to Rust from Python.
- Uses CFFI which allows for compatibility with Pypy.
- Limited in that Python must call out to Rust.
- No coupling to CPython interpreter.

+++


## Oxidizing Python

#### CPython Integration
- Link against Python C headers using PyO3.
- Directly interacts with CPython interpreter and allows two way function calls.
- Potentially (slightly) lower overhead for calling out to Rust.
- Limited to CPython interpreter.
