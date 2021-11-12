use std::{thread, time::Duration};
use throttle_my_fn::throttle;

#[throttle(10, Duration::from_secs(1))]
pub(crate) fn run_10_times_per_second(msg: &str) -> &'static str {
  eprintln!("{}", msg);
  "foo"
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
}
