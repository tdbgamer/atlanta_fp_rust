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

#### Example 3
- In this example instead of asking for ownership, takes_string now asks for an
  immutable borrow, or reference, of the given string.
- After the function runs, the borrow is released, main's scope maintains ownership
  of foo, prints it, then frees it before exiting.

## Zero-cost abstractions
- The Rust compiler is smart enough in many cases to, at compile time, optimize away abstractions that you create.
- To do so, the code must be designed in such a way that as much as possible can be determined at compile-time.
- While Rust provides useful dynamic features like dynamic method dispatch, they should be used sparringly as those
  calls cannot be inlined at compile time and very little optimization can be done.
- In languages like Java, dynamic method dispatch all over the place is fine because the JIT can figure things out at compile time
  that allow it to inline and optimize very dynamic code.
- The downside for some is that this forces developers to break away from the traditional OOP paradigm.

#### Example 1
- This is sort of a tradition OOP example, and this is the sort of Java/C++ OOP way to do it.
- We start off with an array of references to Animals. These _have_ to be references because Vecs are allocated
  as a contiguous chuck of memory, and implementations of Animal can have different sizes in memory.
- The pointer indirection itself has a small inherent cost, but the much larger cost is that calls to `speak()`
  can't be inlined anymore. It is a dynamic call that must happen every time.
- This isn't terribly slow, but if used extensively in hot code paths can be a potential performance issues.
- However in many cases such designs with rigid inheritance hierarchies really aren't a good idea anyways.

#### Example 2
- This is a preferred way to simulate the same behavior in Rust.
- The animals can all be contiguously allocated with no references.
- `speak()` can again be inlined.
- Matching on a Rust enum compiles to a constant time jump table.
- This code is more lean, determines more at compile time, and empowers the compiler to optimize the code well.

#### Example 3
- First take a look at `debug_string`.
- Rust does an optimization called Monomorphization with generics like this.
- I believe in C++ calls this templating.
- Basically the compiler generates a version of this function for each type it's used on.
- For the dynamic version, fat pointers and dynamic dispatch are used instead.
- Dynamic dispatch will generate a single function that runs more slowly and cannot be optimized as aggressively.
- The trade-off between dynamic dispatch and statically dispatched is binary size vs performance.
- Statically dispatch will generate a lot more code, but that code can be aggressively inlined and optimized.
- Storage is cheap, memory is cheap... Use static dispatch as much as possible.
