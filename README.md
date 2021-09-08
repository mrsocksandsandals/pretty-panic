# pretty-panic
> A Rust library to make your ugly panic messages nicer for the end users.

### What is it?
`pretty-panic` is a small library that wraps around [`std::panic::set_hook`](https://doc.rust-lang.org/std/panic/fn.set_hook.html), allowing you to set your own panic handler functions. It provides a default handler, which just prints a nicer panic message.

### How to use?
Code:
```rust
use pretty_panic::pretty_panic;

fn main() {
    pretty_panic!(/* you would put your handler function here */);

    panic!("explicit call to panic!()");
}
```
Output:
```txt
Uh oh!

The program experienced a fatal error, and has panicked. Recommend you contact one of the authors for assistance.
See below for some basic information. If you submit a bug report, be sure to include this entire message.

BEGIN PANIC INFO:
    panicked at 'explicit call to panic!()', src/main.rs 6
