/* ------------------------------------------ */
/* lib.rs - pretty-panic library source code. */
/* ------------------------------------------ */

// Set crate attributes.
#![deny(missing_docs)]

//!
//! # pretty-panic
//!
//! Provides a simple way to deliver prettier panic messages to the user.
//! Wraps around [`std::panic::set_hook()`](https://doc.rust-lang.org/std/panic/fn.set_hook.html), allowing you to easily change
//! the panic message for your release builds.
//!
//! ## Example
//! ```rust
//! use pretty_panic::pretty_panic;
//!
//! fn main() {
//!     pretty_panic!();
//!
//!     panic!("A panic message.");
//! }
//! It's that simple! The `pretty_panic!()` macro can take a custom panic handler function as an argument.
//! If one isn't passed, it simply uses the default.

/// Sets the panic handler function to a function of your choosing.
/// **Example**:
/// ```rust
/// use pretty_panic::pretty_panic;
/// use std::panic::PanicInfo;
/// fn main() {
///     pretty_panic!(my_panic);
///
///     panic!("a panic message")
/// }
///
/// fn my_panic() -> ! {
///     loop { /* who needs to actually do something when they panic? }
/// }
macro_rules! pretty_panic {
    () => {
        if cfg!(release) {
            fn default_handler(e: &PanicInfo) -> ! {
                eprintln!("Uh oh!\
                    \
                    The program experienced a fatal error, and has panicked. Recommend you contact one\
                    of the authors for assistance. See below for some additional information:\
                    \
                    (If you are going to submit a bug report, include the entirety of this message!)\
                    {}"
                );
            }
        }
    }
}
