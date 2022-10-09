use chrono::{Timelike, Utc};
use std::{io::{stdout, Write},thread, time};

fn main() {
    let mut stdout = stdout();
    let offset: u32 = 1;
    let one_second = time::Duration::from_millis(1000);

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    loop {
        let now = Utc::now();
        let mut hour = now.hour() + offset;
    
        if hour > 24 {
            hour = 0;
        }
    
        // print!("\x1B[2J\x1B[1;0H");
        print!(
            "\rThe current time is {:02}:{:02}:{:02}",
            hour,
            now.minute(),
            now.second()
        );

        stdout.flush().unwrap();
        thread::sleep(one_second);
    }
}
