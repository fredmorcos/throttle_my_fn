# `throttle_my_fn`: A Rust attribute macro to throttle the execution of functions

[![License](https://img.shields.io/github/license/fredmorcos/throttle_my_fn?style=for-the-badge)](https://github.com/fredmorcos/throttle_my_fn/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/throttle_my_fn?style=for-the-badge)](https://crates.io/crates/throttle_my_fn)
[![docs.rs](https://img.shields.io/docsrs/throttle_my_fn?style=for-the-badge)](https://docs.rs/throttle_my_fn/0.2.6/throttle_my_fn/)

`throttle_my_fn` is a Rust attribute macro to limit a function's number of runs over a
specified period of time, even when called from multiple threads.

The primary use-case for this attribute macro is rate-limiting, e.g. to avoid hammering an
online service or to avoid serving too many requests over a period of time.

The macro works by rewriting the function and prefixing it with the necessary book-keeping
for throttling (see `Usage` below). **The resulting function is thread-safe**.

## Usage

Add the necessary dependencies to your `Cargo.toml`:

```toml
[dependencies]
throttle_my_fn = "0.2"
parking_lot = "0.11"
```

Or, using `cargo add`:

```sh
$ cargo add throttle_my_fn
$ cargo add parking_lot
```

Include the macro:

```rust
use throttle_my_fn::throttle;
```

Annotate the functions you want to throttle:

```rust
#[throttle(10, Duration::from_secs(1))]
pub(crate) fn run_10_times_per_second(arg: &str) -> String {
  ...
}

#[throttle(1, Duration::from_millis(100))]
pub(crate) fn run_once_per_100_milliseconds(arg: &str) -> String {
  ...
}
```

Note that the function signatures are modified to wrap the return type in an `Option`,
like so:

```rust
pub(crate) fn run_10_times_per_second(arg: &str) -> Option<String> {
  ...
}

pub(crate) fn run_once_per_100_milliseconds(arg: &str) -> Option<String> {
  ...
}
```

The `Option<T>` returned signifies whether the function executed or not.

## Changelog

* 0.2.6
  - Fix compilation error with functions that have 2+ arguments.

* 0.2.5
  - README fixes.

* 0.2.4
  - Ignore example doctests.
  - Update documentation about also adding `parking_lot` as a dependency.

* 0.2.3
  - Add this Changelog section to the README.

* 0.2.2
  - Documentation updates regarding thread-safety.

* 0.2.1
  - Documentation update about possible use cases for this crate.
  - Thanks go to `@not-matthias` from the Rust Linz Discord server for feedback.

* 0.2.0
  - Replace the use of `MaybeUninit` and `AtomicBool` with `parking_lot::Mutex` and
    `parking_lot::const_mutex`.
  - Thanks go to `@mejrs` and `@veber-alex` from the Rust Discord server for feedback.

* 0.1.0
  - Initial release
