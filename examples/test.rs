use std::{thread, time::Duration};
use throttle_my_fn::throttle;

#[throttle(10, Duration::from_secs(1))]
pub(crate) fn run_10_times_per_second(msg: &str) -> &'static str {
  eprintln!("{}", msg);
  "foo"
}

#[throttle(10, Duration::from_secs(1))]
pub(crate) fn run_10_times_per_second_with_2_args(
  msg1: &str,
  msg2: &str,
) -> &'static str {
  eprintln!("msg1={}, msg2={}", msg1, msg2);
  "foo2"
}

#[throttle(10, Duration::from_secs(1))]
pub(crate) fn run_10_times_per_second_with_3_args(
  msg1: &str,
  msg2: &str,
  msg3: &str,
) -> &'static str {
  eprintln!("msg1={}, msg2={}, msg3={}", msg1, msg2, msg3);
  "foo2"
}

#[throttle(1, Duration::from_millis(100))]
pub(crate) fn run_once_per_100_milliseconds(msg: &str) -> &'static str {
  eprintln!("{}", msg);
  "foo"
}

fn main() {
  for _ in 0..3 {
    for i in 0..20 {
      let _ = run_10_times_per_second(&format!("{}: Running 10 times per second", i));
    }

    eprintln!();
    std::thread::sleep(Duration::from_secs(1));
  }

  for _ in 0..3 {
    for i in 0..20 {
      let _ = run_10_times_per_second_with_2_args(
        &format!("{}: Running 10 times per second with 2 args", i),
        "other message",
      );
    }

    eprintln!();
    std::thread::sleep(Duration::from_secs(1));
  }

  for _ in 0..3 {
    for i in 0..20 {
      let _ = run_10_times_per_second_with_3_args(
        &format!("{}: Running 10 times per second with 3 args", i),
        "other message",
        "another message",
      );
    }

    eprintln!();
    std::thread::sleep(Duration::from_secs(1));
  }

  for _ in 0..3 {
    for i in 0..20 {
      let _ = run_once_per_100_milliseconds(&format!(
        "{}: Running once per 100 milliseconds",
        i
      ));
    }

    eprintln!();
    std::thread::sleep(Duration::from_millis(100));
  }

  for _ in 0..3 {
    for i in 0..20 {
      thread::spawn(move || {
        let t = thread::current().id();
        let _ = run_10_times_per_second(&format!(
          "{}: Running 10 times per second from thread {:?}",
          i, t
        ));
      });
    }

    eprintln!();
    std::thread::sleep(Duration::from_secs(1));
  }

  for _ in 0..3 {
    for i in 0..20 {
      thread::spawn(move || {
        let t = thread::current().id();
        let _ = run_10_times_per_second_with_2_args(
          &format!("{}: Running 10 times per second with 2 args from thread {:?}", i, t),
          "other message",
        );
      });
    }

    eprintln!();
    std::thread::sleep(Duration::from_secs(1));
  }

  for _ in 0..3 {
    for i in 0..20 {
      thread::spawn(move || {
        let t = thread::current().id();
        let _ = run_10_times_per_second_with_3_args(
          &format!("{}: Running 10 times per second with 3 args from thread {:?}", i, t),
          "other message",
          "another message",
        );
      });
    }

    eprintln!();
    std::thread::sleep(Duration::from_secs(1));
  }
}
