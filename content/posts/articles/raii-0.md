---
title: RAII all too rusty
date: 2026-07-01
---

# RAII all too rusty - 0
I know most of us know what RAII is but we still gotta talk it for those who aren't aware.

### So, What exactly is it?

It's very simple if you are coming from something like C++:
- acquire a resource `x`
- automatically clean it up when `x` goes out of scope.

So no manual `free()` or `close()`.

Note: I am talking about a resource in general, i.e. not limited to basic Vectors, HashMaps and such.

Rust (🦀):
```rust
{
    let arr = vec![1,2,3,4]; //automatically allocated on heap
}
// as soon as x goes out of scope its cleaned up.
```

Again this is not just limited to datatypes:
```rust
{
    use std::fs::File;

    let file = File::open("foo.txt");
    // do stuff to the file
}
// as soon as file goes out of scope its cleaned up.
```

Rust achieves this by something called the `Drop` trait. Every resource which implements RAII, implements the `Drop` trait.
And whenever the resource goes out scope aka the ownership ends, `drop()` gets called.

You can also implement `Drop` for your own resources.

```rust
struct EatLunch;

impl Drop for EatLunch {
    fn drop(&mut self) {
        println!("Washing hands");
    }
}

fn main() {
    let me = EatLunch; // I eat lunch
}
// As soon as I am done eating, I wash my hands
//because drop() is called.
```
