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
- As a Python developer, I wouldn't trust C written by myself or pretty much anyone at my company.
- On the other hand I _do_ feel confident that anyone I work with could easily create reliable, performance software in Rust.

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

- I will define what it means to _own_ something in Rust in the next section.
- Send types can be safely sent to another thread.
- Sync types can be safely shared between multiple threads.

#### Send/Sync
- Send and Sync are unsafe, marker traits that the compiler uses for to determine thread safety of a type.
- Marker traits have no methods to implement, and are used to indicate some inherent property of a type.
- They are both auto traits that will attempt to implement themselves for all types that it can determine are inherently Send or Sync.
- Manually implementing them is unsafe and is basically telling the compiler, "shutup I'll synchronize it."
- Normally users don't implement Send/Sync themselves, they wrap whatever type they need in wrappers that implement Send/Sync.
- I'll give some examples of those wrappers later.

## Ownership System
- I know this might sound a little confusing so let's go over a few examples
  and then I'll take any questions.

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
- In languages like Java, dynamic method dispatch all over the place is fine because the JIT can figure things out at run-time
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
- Match statement is exhaustive.
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

## Unsafe Code
- Bryan Cantrill referred to unsafe blocks as a sort of conjugal visit with your memory.
- One example of an safe abstraction built over unsafe code is a Mutex wrapper. I'll go over that in a little bit.

#### Mutex example
- Arc means Atomic Reference counted.
- Wrapping some data in an Arc makes it implement Send trait which allows reference counted pointers
  to it to be sent to other threads.
- I'm not able to mutate the variable unless I have a way to synchronize access to it.
- Mutex will wrap any type, synchronize access to it, and implements the Sync trait, which tells the compiler
  that this element is safe to be shared by multiple threads.

- Mutex is a good example of a safe abstraction built on unsafe code.
- Mutating the same memory from multiple threads is unsafe and not allowed by the Rust compiler.
- Locking wrapper types like Mutex allow users to synchronize access such that we know for sure the mutation is safe.


## C Interop
- I believe this is an extremely important component to Rust's success.
- Being able to take advantage of the C ecosystem is a huge deal. 
- There is lots of finely tuned and highly optimized software out there
  that the Rust community can tap into just by making simple safe wrappers around
  old, battle tested, reliable C libraries.
- There is also an entire universe of software that has been implemented in C
  that will _never_  and honestly _should_ never be rewritten in Rust.
- Linux kernel isn't going anywhere, but because Rust has such great two way interop with C,
  people are starting to experiment with writing Linux kernel modules in Rust.
- This is the true power of Rust. We don't have to rewrite the world in Rust, instead we can improve
  what we already have by making components and new features of our software in Rust.
