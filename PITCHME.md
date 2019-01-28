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
- High level languages offer low mental overhead and speed of developement, but aren't usually performant.
    Developers can drop down into Rust where traditionally C had been used.
- Applications with high cost of failure or where correctness and refactorability are valuable.

---

## Usecase: Python

#### Pros
- Approachable for beginners.
- Excellent for fast prototyping while requirements are poorly defined.
- Allow for clean, terse code.
- Plethora of libraries at your fingertips.

+++

## Python

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
- Limited in that Python must call out to Rust, Rust can't call out to Python

+++


## Oxidizing Python

#### CPython Integration
- Link against Python C headers using PyO3.
- Directly interacts with CPython interpreter and allows two way function calls.
- Potentially (slightly) lower overhead for calling out to Rust.
- Limited to CPython interpreter.
