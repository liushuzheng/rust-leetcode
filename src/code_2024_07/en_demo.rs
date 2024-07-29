use std::ops::Add;
use std::thread;
use std::time::{Duration, SystemTime};
use indicatif::ProgressBar;

use tokio::net;

enum Time {
    A(i32),
    B,
    C(SystemTime),
}

pub trait CalTime {
    fn cal(&self) -> SystemTime;
}

impl CalTime for Time {
    fn cal(&self) -> SystemTime {
        let a = match self {
            Time::A(n) => {
                let start = SystemTime::now();
                let duration = Duration::from_days(*n as u64);
                let time = start.add(duration);
                time
            }
            Time::B => {
                SystemTime::now()
            }
            Time::C(s) => {
                return *s;
            }
        };
        a
    }
}

// #[test]
 fn test_aa() {
    // let a =Time::C( SystemTime::now() +Duration::from_days(32));
    let a = Time::A(12);
    let time = a.cal();
    println!("{:?}", time);
}

#[test]
fn test_p(){
    let pb = ProgressBar::new_spinner();
    for _ in 0..1024 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(5));
    }
    pb.finish_with_message("done");
}