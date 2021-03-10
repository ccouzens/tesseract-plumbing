# tesseract-plumbing

Crate to expose a safe version of the
[tesseract-sys](https://crates.io/crates/tesseract-sys) api.

This is designed to expose the C API of
[tesseract](https://github.com/tesseract-ocr/tesseract) in a safe manner.

Adding value by deviating from the API is a non-goal. That is left to libraries
that build on top of `tesseract-plumbing`.

## Motivation

I'm a maintainer of both [leptess](https://crates.io/crates/leptess) and
[tesseract-rs](https://crates.io/crates/tesseract).

I noticed that there was a lot of duplication in how they interact with both
[leptonica-sys](https://crates.io/crates/leptonica-sys) and
[tesseract-sys](https://crates.io/crates/tesseract-sys). Having multiple layers
of abstraction in `leptess` and `tesseract-rs` made it hard to reason about
their memory safety.

Having a safety layer that stays simple improves the correctness and
maintainability of the above libraries.
