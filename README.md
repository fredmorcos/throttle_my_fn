# `throttle_my_fn`: A Rust attribute macro to throttle the execution of functions

[![License](https://img.shields.io/github/license/fredmorcos/throttle_my_fn?style=for-the-badge)](https://github.com/fredmorcos/throttle_my_fn/blob/main/LICENSE)

`throttle_my_fn` is a Rust attribute macro to limit a function's number of runs over a
specified period of time.

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
throttle_my_fn = "0.1"
```

Or, using `cargo add`:

```sh
$ cargo add throttle_my_fn
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
