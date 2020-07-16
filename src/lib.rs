//! [![github]](https://github.com/dtolnay/watt)&ensp;[![crates-io]](https://crates.io/crates/watt)&ensp;[![docs-rs]](https://docs.rs/watt)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! # Watt
//!
//! Watt is a runtime for executing Rust procedural macros compiled as
//! WebAssembly.
//!
//! <br>
//!
//! # Rationale
//!
//! - **Faster compilation.**&emsp;By compiling macros ahead-of-time to Wasm, we
//!   save all downstream users of the macro from having to compile the macro
//!   logic or its dependencies themselves.
//!
//!   Instead, what they compile is a small self-contained Wasm runtime (~3
//!   seconds, shared by all macros) and a tiny proc macro shim for each macro
//!   crate to hand off Wasm bytecode into the Watt runtime (~0.3 seconds per
//!   proc-macro crate you depend on). This is much less than the 20+ seconds it
//!   can take to compile complex procedural macros and their dependencies.
//!
//! - **Isolation.**&emsp;The Watt runtime is 100% safe code with zero
//!   dependencies. While running in this environment, a macro's *only possible
//!   interaction with the world* is limited to consuming tokens and producing
//!   tokens. This is true regardless of how much unsafe code the macro itself
//!   might contain! Modulo bugs in the Rust compiler or standard library, it is
//!   impossible for a macro to do anything other than shuffle tokens around.
//!
//! - **Determinism.**&emsp;From a build system point of view, a macro backed by
//!   Wasm has the advantage that it can be treated as a purely deterministic
//!   function from input to output. There is no possibility of implicit
//!   dependencies, such as via the filesystem, which aren't visible to or taken
//!   into account by the build system.
//!
//! <br>
//!
//! # Getting started
//!
//! Start by implementing and testing your proc macro as you normally would,
//! using whatever dependencies you want (syn, quote, etc). You will end up with
//! something that looks like:
//!
//! ```
//! # const IGNORE: &str = stringify! {
//! use proc_macro::TokenStream;
//!
//! #[proc_macro]
//! pub fn the_macro(input: TokenStream) -> TokenStream {
//!     /* ... */
//! }
//! # };
//! ```
//!
//! `#[proc_macro_derive]` and `#[proc_macro_attribute]` are supported as well;
//! everything is analogous to what will be shown here for `#[proc_macro]`.
//!
//! When your macro is ready, there are just a few changes we need to make to
//! the signature and the Cargo.toml. In your lib.rs, change each of your macro
//! entry points to a no\_mangle extern "C" function, and change the TokenStream
//! in the signature from proc\_macro to proc\_macro2.
//!
//! It will look like:
//!
//! ```
//! # const IGNORE: &str = stringify! {
//! use proc_macro2::TokenStream;
//!
//! #[no_mangle]
//! pub extern "C" fn the_macro(input: TokenStream) -> TokenStream {
//!     /* same as before */
//! }
//! # };
//! ```
//!
//! Now in your macro's Cargo.toml which used to contain this:
//!
//! ```toml
//! [lib]
//! proc-macro = true
//! ```
//!
//! change it instead to say:
//!
//! ```toml
//! [lib]
//! crate-type = ["cdylib"]
//!
//! [patch.crates-io]
//! proc-macro2 = { git = "https://github.com/dtolnay/watt" }
//! ```
//!
//! This crate will be the binary that we compile to Wasm. Compile it by
//! running:
//!
//! ```console
//! $ cargo build --release --target wasm32-unknown-unknown
//! ```
//!
//! Next we need to make a small proc-macro shim crate to hand off the compiled
//! Wasm bytes into the Watt runtime. It's fine to give this the same crate name
//! as the previous crate, since the other one won't be getting published to
//! crates.io. In a new Cargo.toml, put:
//!
//! ```toml
//! [lib]
//! proc-macro = true
//!
//! [dependencies]
//! watt = "0.4"
//! ```
//!
//! And in its src/lib.rs, define real proc macros corresponding to each of the
//! ones previously defined as no\_mangle extern "C" functions in the other
//! crate:
//!
//! ```
//! # const IGNORE: &str = stringify! {
//! use proc_macro::TokenStream;
//! use watt::WasmMacro;
//!
//! static MACRO: WasmMacro = WasmMacro::new(WASM);
//! static WASM: &[u8] = include_bytes!("my_macros.wasm");
//!
//! #[proc_macro]
//! pub fn the_macro(input: TokenStream) -> TokenStream {
//!     MACRO.proc_macro("the_macro", input)
//! }
//! # };
//! ```
//!
//! Finally, copy the compiled Wasm binary from
//! target/wasm32-unknown-unknown/release/my_macros.wasm under your
//! implementation crate, to the src directory of your shim crate, and it's
//! ready to publish!
//!
//! <br>
//!
//! # Remaining work
//!
//! - **Performance.**&emsp;Watt compiles pretty fast, but so far I have not put
//!   any effort toward optimizing the runtime. That means macro expansion can
//!   potentially take longer than with a natively compiled proc macro.
//!
//!   Note that the performance overhead of the Wasm environment is partially
//!   offset by the fact that our proc macros are compiled to Wasm in release
//!   mode, so downstream `cargo build` will be running a release-mode macro
//!   when it would have been running debug-mode for a traditional proc macro.
//!
//!   A neat approach would be to provide some kind of `cargo install
//!   watt-runtime` which installs an optimized Wasm runtime locally, which the
//!   Watt crate can detect and hand off code to if available. That way we avoid
//!   running things in a debug-mode runtime altogether. The experimental
//!   beginnings of this can be found under the [jit/] directory.
//!
//! - **Tooling.**&emsp;The getting started section shows there are a lot of
//!   steps to building a macro for Watt, and a pretty hacky patching in of
//!   proc-macro2. Ideally this would all be more straightforward, including
//!   easy tooling for doing reproducible builds of the Wasm artifact for
//!   confirming that it was indeed compiled from the publicly available
//!   sources.
//!
//! - **RFCs.**&emsp;The advantages of fast compile time, isolation, and
//!   determinism may make it worthwhile to build first-class support for Wasm
//!   proc macros into rustc and Cargo. The toolchain could ship its own high
//!   performance Wasm runtime, which is an even better outcome than Watt
//!   because that runtime can be heavily optimized and consumers of macros
//!   don't need to compile it.
//!
//! [jit/]: https://github.com/dtolnay/watt/tree/master/jit
//!
//! <br>
//!
//! # Acknowledgements
//!
//! The current underlying Wasm runtime is a fork of the [Rust-WASM] project by
//! Yoann Blein and Hugo Guiroux, a simple and spec-compliant WebAssembly
//! interpreter.
//!
//! [Rust-WASM]: https://github.com/yblein/rust-wasm
#![deny(rust_2018_idioms)]

extern crate proc_macro;

#[cfg(not(jit))]
#[path = "interpret.rs"]
mod exec;

#[cfg(not(jit))]
#[path = "../runtime/src/lib.rs"]
mod runtime;

#[cfg(jit)]
#[path = "jit.rs"]
mod exec;

#[cfg(jit)]
#[path = "../jit/src/lib.rs"]
mod runtime;

mod data;
mod decode;
mod encode;
mod import;
mod sym;

use proc_macro::TokenStream;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

/// An instantiation of a WebAssembly module used to invoke procedural macro
/// methods on the wasm module.
///
///
/// # Examples
///
/// ```
/// # const IGNORE: &str = stringify! {
/// static MACRO: WasmMacro = WasmMacro::new(WASM);
/// static WASM: &[u8] = include_bytes!("my_macros.wasm");
/// # };
/// ```
pub struct WasmMacro<'bytecode> {
    wasm: &'bytecode [u8],
    id: AtomicUsize,
}

impl<'bytecode> WasmMacro<'bytecode> {
    /// Creates a new `WasmMacro` from the statically included blob of wasm bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # const IGNORE: &str = stringify! {
    /// static MACRO: WasmMacro = WasmMacro::new(WASM);
    /// static WASM: &[u8] = include_bytes!("my_macros.wasm");
    /// # };
    /// ```
    pub const fn new(wasm: &'bytecode [u8]) -> Self {
        WasmMacro {
            wasm,
            id: AtomicUsize::new(0),
        }
    }

    /// A #\[proc_macro\] implemented in wasm!
    ///
    /// # Canonical macro implementation:
    ///
    /// ```
    /// # const IGNORE: &str = stringify! {
    /// use proc_macro2::TokenStream;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn the_macro(input: TokenStream) -> TokenStream {
    ///     ...
    /// }
    /// # };
    /// ```
    ///
    /// # Canonical entry point:
    ///
    /// ```
    /// # const IGNORE: &str = stringify! {
    /// use proc_macro::TokenStream;
    /// use watt::WasmMacro;
    ///
    /// static MACRO: WasmMacro = WasmMacro::new(WASM);
    /// static WASM: &[u8] = include_bytes!("my_macros.wasm");
    ///
    /// #[proc_macro]
    /// pub fn the_macro(input: TokenStream) -> TokenStream {
    ///     MACRO.proc_macro("the_macro", input)
    /// }
    /// # };
    /// ```
    pub fn proc_macro(&self, fun: &str, input: TokenStream) -> TokenStream {
        exec::proc_macro(fun, vec![input], self)
    }

    /// A #\[proc_macro_derive\] implemented in wasm!
    ///
    /// # Canonical macro implementation:
    ///
    /// ```
    /// # const IGNORE: &str = stringify! {
    /// use proc_macro2::TokenStream;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn the_macro(input: TokenStream) -> TokenStream {
    ///     ...
    /// }
    /// # };
    /// ```
    ///
    /// # Canonical entry point:
    ///
    /// ```
    /// # const IGNORE: &str = stringify! {
    /// use proc_macro::TokenStream;
    /// use watt::WasmMacro;
    ///
    /// static MACRO: WasmMacro = WasmMacro::new(WASM);
    /// static WASM: &[u8] = include_bytes!("my_macros.wasm");
    ///
    /// #[proc_macro_derive(MyDerive)]
    /// pub fn the_macro(input: TokenStream) -> TokenStream {
    ///     MACRO.proc_macro_derive("the_macro", input)
    /// }
    /// # };
    /// ```
    pub fn proc_macro_derive(&self, fun: &str, input: TokenStream) -> TokenStream {
        exec::proc_macro(fun, vec![input], self)
    }

    /// A #\[proc_macro_attribute\] implemented in wasm!
    ///
    /// # Canonical macro implementation:
    ///
    /// ```
    /// # const IGNORE: &str = stringify! {
    /// use proc_macro2::TokenStream;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn the_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    ///     ...
    /// }
    /// # };
    /// ```
    ///
    /// # Canonical entry point:
    ///
    /// ```
    /// # const IGNORE: &str = stringify! {
    /// use proc_macro::TokenStream;
    /// use watt::WasmMacro;
    ///
    /// static MACRO: WasmMacro = WasmMacro::new(WASM);
    /// static WASM: &[u8] = include_bytes!("my_macros.wasm");
    ///
    /// #[proc_macro_attribute]
    /// pub fn the_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    ///     MACRO.proc_macro_attribute("the_macro", args, input)
    /// }
    /// # };
    /// ```
    pub fn proc_macro_attribute(
        &self,
        fun: &str,
        args: TokenStream,
        input: TokenStream,
    ) -> TokenStream {
        exec::proc_macro(fun, vec![args, input], self)
    }

    fn id(&self) -> usize {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
        match self.id.load(SeqCst) {
            0 => {}
            n => return n,
        }
        let id = NEXT_ID.fetch_add(1, SeqCst);
        self.id
            .compare_exchange(0, id, SeqCst, SeqCst)
            .unwrap_or_else(|id| id)
    }
}
