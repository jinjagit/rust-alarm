use chrono::{Timelike, Utc};
use std::{thread, time};

fn main() {
    let offset: u32 = 1;
    let one_second = time::Duration::from_millis(1000);

    loop {
        let now = Utc::now();
        let mut hour = now.hour() + offset;
    
        if hour > 24 {
            hour = 0;
        }
    
        println!(
            "The current time is {:02}:{:02}:{:02}",
            hour,
            now.minute(),
            now.second()
        );

        thread::sleep(one_second);
    }
}
