use chrono::{Timelike, Utc};
use std::{io::{stdout}, thread, time};
use crossterm::{ExecutableCommand, cursor};

fn main() {
    let mut stdout = stdout();
    let offset: u32 = 1;
    let one_second = time::Duration::from_millis(1000);

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear terminal
    stdout.execute(cursor::Hide).ok();

    loop {
        let now = Utc::now();
        let mut hour = now.hour() + offset;
    
        if hour > 24 {
            hour = 0;
        }
    
        // print!("\x1B[2J\x1B[1;0H");
        stdout.execute(cursor::MoveTo(0,0)).ok();

        println!(
            "The current time is {:02}:{:02}:{:02}",
            hour,
            now.minute(),
            now.second()
        );

        println!();
        println!("something else");

        thread::sleep(one_second);
    }
}
