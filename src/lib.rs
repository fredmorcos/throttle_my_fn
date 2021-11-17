//! `throttle_my_fn` is a Rust attribute macro to limit a function's number of runs over a
//! specified period of time.
//!
//! ## Usage
//!
//! Add the dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! throttle_my_fn = "0.1"
//! ```
//!
//! Or, using `cargo add`:
//!
//! ```sh
//! $ cargo add throttle_my_fn
//! ```
//!
//! Include the macro:
//!
//! ```rust
//! use throttle_my_fn::throttle;
//! ```
//!
//! Annotate the functions you want to throttle:
//!
//! ```rust
//! #[throttle(10, Duration::from_secs(1))]
//! pub(crate) fn run_10_times_per_second(arg: &str) -> String {
//!   ...
//! }
//!
//! #[throttle(1, Duration::from_millis(100))]
//! pub(crate) fn run_once_per_100_milliseconds(arg: &str) -> String {
//!   ...
//! }
//! ```
//!
//! Note that the function signatures are modified to wrap the return type in an `Option`,
//! like so:
//!
//! ```rust
//! pub(crate) fn run_10_times_per_second(arg: &str) -> Option<String> {
//!   ...
//! }
//!
//! pub(crate) fn run_once_per_100_milliseconds(arg: &str) -> Option<String> {
//!   ...
//! }
//! ```
//!
//! The `Option<T>` returned signifies whether the function executed or not.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::fmt::Display;
use syn::parse::Parser;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, ItemFn, ReturnType, Token};

/// Shorthand for creating `syn::Error`s that type-check with [proc_macro::TokenStream].
///
/// Creates fancy-looking error messages on stable Rust that can be returned from
/// functions that return a [proc_macro::TokenStream].
///
/// # Arguments
///
/// * `tokens` - The [proc_macro::TokenStream] to be spanned in the error message.
///
/// * `message` - Custom user-provided error message.
///
/// # Returns
///
/// A [proc_macro::TokenStream], but effectively returns nothing and stops execution. It
/// uses [syn::parse::Error::into_compile_error] which is akin to `compile_error!()`.
fn err<T, U>(tokens: T, message: U) -> TokenStream
where
  T: ToTokens,
  U: Display,
{
  syn::Error::new_spanned(tokens, message).into_compile_error().into()
}

/// Throttle a function's execution count over a period of time.
///
/// Slow down how many times a function can be executed over a duration: 100 times per
/// [std::time::Duration::from_secs]`(1)` for 100 times per second.
///
/// **NOTE:** The function being decorated with this macro will have its return value
/// changed to be wrapped in an [std::option::Option] indicating whether the function
/// executed or not.
///
/// # Arguments
///
/// * `times` - Number of times the function should be limited to running over `duration`.
///
/// * `duration` - The [std::time::Duration] over which the function should be allowed to
/// run `times` times.
///
/// # Examples
///
/// ```
/// #[throttle(10, Duration::from_secs(1))]
/// pub(crate) fn run_10_times_per_second(arg: &str) -> String {
///   ...
/// }
///
/// #[throttle(1, Duration::from_millis(100))]
/// pub(crate) fn run_once_per_100_milliseconds(arg: &str) -> String {
///   ...
/// }
/// ```
#[proc_macro_attribute]
pub fn throttle(args: TokenStream, func: TokenStream) -> TokenStream {
  // How this macro works is by wrapping the user-provided function (called the impl here)
  // in an outer function with a similar signature. The return type T of the impl function
  // is changed to an Option<T> on the outer function to indicate whether the function
  // executed or not.
  //
  // The outer function then initializes the necessary statics, does the book-keeping,
  // then decides whether the execute the impl or not.

  const ARGS_ERR_MSG: &str = "expecting a comma-separated pair of expressions: \
                              #[throttle(<number-of-calls>, <duration>)]";

  let args_parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
  let args_parsed = match args_parser.parse(args.clone()) {
    Ok(args) => args,
    Err(e) => return err(TokenStream2::from(args), &format!("{}, {}", e, ARGS_ERR_MSG)),
  };

  let mut args_iter = args_parsed.iter();

  let times = match args_iter.next() {
    Some(times) => times,
    None => return err(TokenStream2::from(args), ARGS_ERR_MSG),
  };

  let duration = match args_iter.next() {
    Some(duration) => duration,
    None => return err(TokenStream2::from(args), ARGS_ERR_MSG),
  };

  // Clone func and operate on the clone so we can move it later for creating error
  // messages with spans.
  let func_clone = func.clone();
  let func_parsed = parse_macro_input!(func_clone as ItemFn);

  let attrs = &func_parsed.attrs;
  let vis = &func_parsed.vis;
  let impl_block = &func_parsed.block;

  // Rename the impl function's name from FUNC_NAME to __throttle_impl_FUNC_NAME. Not
  // really necessary, and could have just been renamed to inner_impl or something like
  // that, since impl is an inner function inside of the outer function.
  let mut impl_sig = func_parsed.sig.clone();
  let impl_ident_name = &format!("__throttle_impl_{}", impl_sig.ident);
  let impl_ident = Ident::new(impl_ident_name, impl_sig.ident.span());
  impl_sig.ident = impl_ident.clone();

  // Change the outer function's signature to be the same as the inner impl function's
  // signature, except for its return type: change that to return an Option<T>.
  let mut outer_sig = func_parsed.sig.clone();
  let outer_sig_ret = TokenStream::from(match outer_sig.output {
    ReturnType::Default => quote! { -> Option<()>},
    ReturnType::Type(_, t) => quote! { -> Option<#t>},
  });
  outer_sig.output = parse_macro_input!(outer_sig_ret);

  // Create the list of arguments for passing the outer function's arguments to the inner
  // impl function.
  let mut call_params = Punctuated::<Expr, Token![,]>::new();
  for input in &impl_sig.inputs {
    match input {
      syn::FnArg::Receiver(_) => {
        return err(TokenStream2::from(func), "Methods are not supported")
      }
      syn::FnArg::Typed(t) => {
        let pat = &t.pat;
        let param = TokenStream::from(quote! {#pat});
        let param = parse_macro_input!(param as Expr);
        call_params.push(param);
      }
    }
  }
  let call_params = call_params.iter();

  // Finally generate our code.
  let gen = quote! {
    // The outer function with an Option<T> return type.
    #(#attrs)* #vis #outer_sig {
      // The inner impl function. Pretty much the user provided one without any visibility
      // modifiers.
      #impl_sig #impl_block

      use std::mem::MaybeUninit;
      use std::sync::atomic::{AtomicBool, Ordering};
      use std::sync::{Arc, Mutex};
      use std::time::Instant;
      use std::collections::VecDeque;

      // We maintain a list of timestamps at which calls to the function have happened in
      // the `calls` deque. This function cleans the deque up by removing all calls that
      // happened before `current_time` - `duration`. The deque should never grow larger
      // than `times`.
      fn cleanup(calls: &mut VecDeque<Instant>, current_time: Instant) {
        if calls.len() < #times {
          return;
        }

        while let Some(call_time) = calls.front().copied() {
          if current_time.duration_since(call_time) > #duration {
            let _ = calls.pop_front();
          } else {
            break;
          }
        }
      }

      let current_time = Instant::now();

      static mut CALLS: MaybeUninit<Arc<Mutex<VecDeque<Instant>>>> = MaybeUninit::uninit();
      static INITIALIZED: AtomicBool = AtomicBool::new(false);

      // Initialize our deque of call timestamps if it's not already initialized.
      if let Ok(false) = INITIALIZED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
        unsafe { CALLS.write(Arc::new(Mutex::new(VecDeque::with_capacity(#times)))) };
      }

      // Lock access to and cleanup our calls deque.
      let mut calls_guard = unsafe { CALLS.as_mut_ptr().as_mut() }.unwrap().lock().unwrap();
      cleanup(&mut calls_guard, current_time);

      // Return None if our quota is full for the duration.
      if calls_guard.len() >= #times {
        return None;
      }

      calls_guard.push_back(current_time);

      // Drop the lock here so that other threads can call us even while the inner impl
      // function is running.
      drop(calls_guard);

      Some(#impl_ident(#(#call_params)*))
    }
  };

  TokenStream::from(gen)
}
