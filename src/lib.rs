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
//! ```
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
///     loop { /* who needs to actually do something when they panic? */ }
/// }
/// ```
/// The panic handler MUST return the [`never`](https://doc.rust-lang.org/std/primitive.never.html) type, or the program will not compile!
#[macro_export]
macro_rules! pretty_panic {
    () => {
        use std::panic::{set_hook, PanicInfo};
        fn default_handler(e: &PanicInfo) -> ! {
            use std::thread;
            let pkg_name: String = env!("CARGO_PKG_NAME").into();
            let pkg_ver: String = env!("CARGO_PKG_VERSION").into();
            let pkg_devs: String = env!("CARGO_PKG_AUTHORS").replace(":", ", ").into();
            let pkg_home: String = env!("CARGO_PKG_HOMEPAGE").into();
            let t = thread::current();
            let t = match t.name() {
                Some(name) => name,
                None => "unknown",
            };

            eprintln!("Uh oh!\n");
            eprintln!("The program experienced a fatal error, and has panicked. Recommend you contact one");
            eprintln!("of the authors for assistance. See below for some additional information:\n");
            eprintln!("(If you are going to submit a bug report, include the entirety of this message!)");
            eprintln!("{} v{} ({}) - panic start", pkg_name, pkg_ver, pkg_home);
            eprintln!("     panic from thread [{}]:\n         {}\n", t, e);
            eprintln!("Submit bug report to the authors: {}", pkg_devs);
            eprintln!("{} v{} ({}) - panic end", pkg_name, pkg_ver, pkg_home);
            std::process::exit(101);
        }
        set_hook(Box::new(|e| { default_handler(e) }));
    };

    ($fname:ident) => {
            use std::panic::{set_hook, PanicInfo};
            set_hook(Box::new(|e| { $fname(e) }));
    }
}
