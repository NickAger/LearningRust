# LearningRust
* https://serokell.io/blog/rust-for-haskellers

Focusing on embedded uses of [Rust](https://www.rust-lang.org) see:
* [This is a three day Rust course developed by the Android team](https://google.github.io/comprehensive-rust/welcome.html) - covers some Bare metal embedded work as well as concurrency.
* [The Rust Programming Language - standard online text](https://doc.rust-lang.org/book/index.html)
* [Async in Rust](https://aturon.github.io/apr/async-in-rust/chapter.html)
* [Rust and Microcontrollers](https://rust-embedded.github.io/discovery/) - embedded systems with Rust.
* [The Rustonomicon - The Dark Arts of Advanced and Unsafe Rust Programming](https://doc.rust-lang.org/nomicon/)
* [Rust Koans](https://github.com/crazymykl/rust-koans)
* [Rustlings](https://github.com/rustlings/rustlings) - Small exercises to get you used to reading and writing Rust code
* [Bare-metal Raspberry Pi 3 tutorials in Rust](https://github.com/rust-embedded/rust-raspi3-tutorial)
* [FP Complete Rust crash course](https://www.snoyman.com/blog/2018/10/introducing-rust-crash-course)
* associated [wiki](https://github.com/NickAger/LearningRust/wiki)
* [C++ & Rust: (Interior) Mutability, Moving and Ownership](https://www.tangramvision.com/blog/c-rust-interior-mutability-moving-and-ownership)

# Rust
Rust a language that has "speed, correctness, and expressiveness" in the same language.

Links:
* CLIs: https://rust-lang-nursery.github.io/cli-wg/
* WASM: https://rustwasm.github.io/book/
* Network services: https://aturon.github.io/apr/
* Embedded: https://rust-embedded.github.io/book/

## Borrow checker

> The basic idea of the borrow checker is that values may not be mutated or moved while they are borrowed, but how do we know whether a value is borrowed? The idea is quite simple: whenever you create a borrow, the compiler assigns the resulting reference a lifetime. This lifetime corresponds to the span of the code where the reference may be used. The compiler will infer this lifetime to be the smallest lifetime that it can have that still encompasses all the uses of the reference.

From: [Non lexical lifetimes RFC](https://github.com/nikomatsakis/nll-rfc/blob/master/0000-nonlexical-lifetimes.md#how-we-teach-this)

## Swift on a Cortex M3
Alternatives:
* [Presentation from Steven Gray - “Programming the Internet of Things with Swift”](SwiftThings.pdf)


