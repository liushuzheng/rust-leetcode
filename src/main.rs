use tokio;

// fn main() {
//     // println!("Hello, world!");
//     // 创建runtime
//     let rt = tokio::runtime::Runtime::new().unwrap();
// }

use std::convert::TryInto;
use std::thread;
use std::time::Duration;
use indicatif::ProgressBar;

fn main() {
    let pb = ProgressBar::new(1024);
    for _ in 0..1024 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(5));
    }
    pb.finish_with_message("done");
}
