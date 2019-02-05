<!-- $theme: gaia -->

# Introduction to @color[orange](**Rust**)
#### by [Tim Bess](https://github.com/tdbgamer)

---

## Brief History

- Started at Mozilla in 2006.
- Mozilla sponsored the language in hopes it could be used to improve Firefox.
- Went on to be used to build Servo.
- Pieces of Servo were put into Firefox in the FF Quantum release.
- Browser overall speed was massively improved.

---

## What is Rust?
- @color[orange](**Non-GC**)'d systems level language.
- Focused on @color[orange](**safety**), @color[orange](**performance**), @color[orange](**concurrency**), and @color[orange](**interoperability**).
- Designed to make systems level software more accessible to developers.

---

## When is Rust Useful?
- When C-level performance is needed, but without the associated low level stumbling blocks.
- Applications with high cost of failure or where correctness and refactorability are valuable.
- High level languages offer low mental overhead and speed of development, but aren't usually performant.
    Developers can drop down into Rust where traditionally C has been used.

---

## Rust Features

- Will not @color[orange](**segfault**).
- Will not allow many classes of @color[orange](**data races**).
- Will not allow @color[orange](**memory leaks**).
- @color[orange](**Zero-cost**) abstractions.

---

## Safe references
- Pointer aliasing and mutability are not mixed (compile-time guarantee).
- Pointers never live longer than the data they hold.
- Array types provide safe bounds checking, unlike C.
- Nulls cannot be created or accidentally dereferenced.

+++

## Safe references

```rust
// Option is defined in stdlib as
pub enum Option<T> {
    None,
    Some(T),
}
```

```rust
fn get_value() -> Option<String> {
    Some(String::from("Foobar"))
}

fn main() {
    // Value may either be None, or a Some(string)
    let foo = get_value();
    
    // Extract the value from the enum
    match foo {
        Some(val) => { println!("{}", val); },
        None => { println!("No value!"); },
    }
}
```

---

## Data races
- Rust guarantees pointers must be @color[orange](exclusively aliased) or @color[orange](mutated).
- Send/Sync traits define whether types can be sent between threads and/or shared between threads respectively.
- Most structs that @color[orange](_own_) their fields automatically implement Send.
- This prevents _many_ types of data races.

+++

## Data races

#### Send/Sync

```rust
// Stdlib definition of Send and Sync
pub unsafe auto trait Send { }
pub unsafe auto trait Sync { }
```

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

## Zero-cost abstractions

- Rust compiler offers strict guarantees about lifetimes, mutability, etc. that allow it to do big compile time optimizations.
- Many high level abstractions, like futures and iterators, are completely torn out at compile time.

+++

## Zero-cost abstractions

#### Example 1
```rust
trait Animal {
    fn speak(&self) -> String;
}
```

```rust
struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) -> String { "Woof".to_string() }
}
impl Animal for Cat {
    fn speak(&self) -> String { "Meow".to_string() }
}
fn main() {
    let animals: Vec<&dyn Animal> = vec![&Dog, &Cat];
    println!("{:?}", animals.iter().map(|x| x.speak())
                            .collect::<Vec<String>>());
}
```

+++

## Zero-cost abstractions

#### Example 2

```rust
enum Animal { Dog, Cat }
impl Animal {
    fn speak(&self) -> String {
        match self {
            Animal::Dog => "Woof".to_string(),
            Animal::Cat => "Meow".to_string()
        }
    }
}
fn main() {
    use Animal::*;
    let animals: Vec<Animal> = vec![Dog, Cat];
    println!("{:?}", animals.iter().map(|x| x.speak())
                            .collect::<Vec<String>>());
}
```

+++

## Zero-cost abstractions

#### Example 3
```rust
use std::fmt::Debug;
fn debug_string<T: Debug>(obj: T) -> String {
    format!("{:?}", obj)
}
fn debug_string_dynamic(obj: &dyn Debug) -> String {
    format!("{:?}", obj)
}
fn main() {
    println!("{}", debug_string(vec![1, 2, 3]));
    println!("{}", debug_string("foobar"));
    println!("{}", debug_string_dynamic(&vec![1, 2, 3]));
    println!("{}", debug_string_dynamic(&"foobar"));
}
```

---

## Unsafe code
- Allows for raw pointers, aliasing, mutation, etc. All the things the Rust compiler normally prevents.
- Unsafe should not be used to bypass safety guarantees, but instead to provide safe abstractions over unsafe code.
- Many stdlib types that offer safe abstractions are built on unsafe code that has been hand verified.
- Necessary for FFI code.

+++


+++

## Safe Unsafe Code

#### Mutex example
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let mutex = Arc::new(Mutex::new(0));
  let c_mutex = mutex.clone();

  thread::spawn(move || {
      *c_mutex.lock().unwrap() = 10;
  }).join().expect("thread::spawn failed");
  assert_eq!(*mutex.lock().unwrap(), 10);
}
```

---

## C Interop
- Rust has two way FFI support with C.
- Structs can optionally use C layout so that C code can read/mutate Rust structs.
- Bindgen can be used to generate Rust definitely for C foreign functions.
- Compatible with C ABI and can generate C header files for Rust libs through CBindgen.
- Because CBindgen generates C header files, many interpreted languages can call out to Rust just like C.

---

## Live coding!
Feel free to code along with me using the [Rust playground](https://play.rust-lang.org/) or by [installing rustup](https://www.rust-lang.org/tools/install).

---

## Usecase: Python

#### Pros
- Approachable for beginners.
- Excellent for fast prototyping while requirements are poorly defined.
- Allows for clean, terse code.
- Plethora of libraries at your fingertips.

+++

## Usecase: Python

#### Cons
- GIL limits parallel processing.
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
