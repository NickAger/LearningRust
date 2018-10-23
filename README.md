# LearningRust
Focusing on embedded uses of [Rust](https://www.rust-lang.org) see:
* [The Rust Programming Language - standard online text](https://doc.rust-lang.org/book/index.html)
* [Rust and Microcontrollers](https://rust-embedded.github.io/discovery/) - embedded systems with Rust.
* [The Rustonomicon - The Dark Arts of Advanced and Unsafe Rust Programming](https://doc.rust-lang.org/nomicon/)
* [Rust Koans](https://github.com/crazymykl/rust-koans)
* associated [wiki](https://github.com/NickAger/LearningRust/wiki)

# Rust
Rust a language that has "speed, correctness, and expressiveness" in the same language.
## Borrow checker

> The basic idea of the borrow checker is that values may not be mutated or moved while they are borrowed, but how do we know whether a value is borrowed? The idea is quite simple: whenever you create a borrow, the compiler assigns the resulting reference a lifetime. This lifetime corresponds to the span of the code where the reference may be used. The compiler will infer this lifetime to be the smallest lifetime that it can have that still encompasses all the uses of the reference.

From: [Non lexical lifetimes RFC](https://github.com/nikomatsakis/nll-rfc/blob/master/0000-nonlexical-lifetimes.md#how-we-teach-this)

## Swift on a Cortex M3
Alternatives:
* [Presentation from Steven Gray - “Programming the Internet of Things with Swift”](SwiftThings.pdf)


