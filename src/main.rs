mod display_data;
use display_data::display_data;
use chrono::{Timelike, Utc};
use std::{io::stdout, thread, time};
use crossterm::{ExecutableCommand, cursor};

fn main() {
    let mut stdout = stdout();
    let display_data = display_data();
    let offset: i32 = 1;
    let one_second = time::Duration::from_millis(1000);

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear terminal
    stdout.execute(cursor::Hide).ok();

    loop {
        let now = Utc::now();
        let mut hour:i32 = now.hour() as i32 + offset;
    
        if hour > 24 {
            hour = hour - 24;
        }

        if hour < 0 {
            hour = 24 - hour;
        }
    
        stdout.execute(cursor::MoveTo(0,0)).ok();

        println!(
            "The current time is {:02}:{:02}:{:02}",
            hour,
            now.minute(),
            now.second()
        );

        println!();
        println!("display_data: {:?}", display_data);

        thread::sleep(one_second);
    }
}
