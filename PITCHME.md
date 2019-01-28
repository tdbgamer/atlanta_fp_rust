<!-- $theme: gaia -->

# Introduction to @color[orange](**Rust**)
#### Created by [Tim Bess](https://github.com/tdbgamer)


---

# What is Rust
- @color[orange](**Non-GC**)'d systems level language.
- Focused on @color[orange](**safety**), @color[orange](**performance**), @color[orange](**concurrency**), and @color[orange](**interoperability**).
- Designed to make systems level software more accessible to developers.

---

# Why do I need Rust
High level languages offer low mental overhead and speed of developement, but aren't usually performant.

---

# Python

#### Pros
- Approachable for beginners.
- Excellent for fast prototyping while requirements are poorly defined.
- Allow for clean, terse code.
- Plethora of libraries at your fingertips.

+++

#### Cons
- Very slow
    - pypy is making things better, but still needs work to be compatible certain python libraries.
    - With scientific python, users have to offload as much work as possible onto pandas/numpy to be performant.
- Difficult to refactor since type hints were added fairly recently and aren't commonly used.
    - Efforts like Mypy might improve this in the future, but would require mass adoption to be useful in many cases.
