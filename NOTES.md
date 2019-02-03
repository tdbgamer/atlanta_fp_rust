## Intro

Hi, I'm Tim.
I used to work a startup called Predikto that was recently acquired by UTC, and I still work at UTC as a Software Engineer currently.

Without further ado, let's get started.

## Brief History

- First ask how many people have heard of Rust and used it.
- Rust started as a Mozilla employee's side project the grew into the language to be used
  to build Servo, out of which would come Firefox Quantum.
- Mozilla has slowly started writing more and more pieces of Firefox in Rust.
- Because Rust can prevent entire classes of security issues, those pieces are much easier to maintain and harder to exploit.

## When is Rust Useful
- Python is slow.
- I won't say it's impossible, but it's extremely unlikely a Python developer will write good C.
- Rust can be used as a viable replacement in these cases.
- As a Python developer, I would feel much more confident in my Rust code vs. my C code.

## Rust Features
- I'll go into each of these in a bit more detail over the next slides.
- At this point anyone who's written C is probably surprised or skeptical, and anyone whose written Python probably thinks, "yeah big deal, no segfaults or memory leaks wooo."
- But the significants comes with the low level control and performance you gain with Rust.

## Safe references
#### Page 1
- By applying these rules, Rust is able to guarantee you won't write software that segfaults.

#### Page 2
- Instead of Null rust provides and Option enum that can be either Something (along with a value), or Nothing.
- People familiar with Haskell will know this as the Maybe monad.
- Because absense of value can be represented in the type system,
  it forces users to check for nothing when an option is returned and to handle that case.
- This is better than in other OOP languages where null can be given in place of any object and can cause downstream
  NullPointerExceptions later.

## Data races

- I will define what it means to _own_ something in Rust on the next slide.

## Ownership System

#### Example 1
- In this example foo is owned within that short scope.
  Accoring to the ownership rules, the value is dropped after going out of scope.
- This example is pretty straight forward to anyone used to languages with lexical scoping.

#### Example 2
- This example is much less straight forward to anyone who hasn't used Rust before.
- take_string is actually asking for ownership of any string that is passed to it.
- After the function runs, it actually drops the String and frees it.
- This fails to compile because foo no longer exists after take_string runs.
- C/C++ programmers are probably confused, because in C this would copy the struct or class.
- Programmers coming from GC'd languages are likely confused as well because they
  aren't used to having to care about when objects are destroyed.
- The good thing about this is that Rust doesn't let you make that mistake and ensures the memory is freed.

### Example 3
- In this example instead of asking for ownership, takes_string now asks for an
  immutable borrow, or reference, of the given string.
- After the function runs, the borrow is released, main's scope maintains ownership
  of foo, prints it, then frees it before exiting.

## Zero-cost abstractions
