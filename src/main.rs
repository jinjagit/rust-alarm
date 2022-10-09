mod display_data;
use display_data::display_data;
use chrono::{Timelike, Utc, DateTime};
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
        let hour:i32 = correct_hour_with_offset(now.hour() as i32 + offset);
    
        stdout.execute(cursor::MoveTo(0,0)).ok();

        time_display(now, hour, display_data.clone());

        thread::sleep(one_second);
    }
}

fn correct_hour_with_offset(mut hour: i32) -> i32 {
    if hour > 24 {
        hour = hour - 24;
    }

    if hour < 0 {
        hour = 24 - hour;
    }

    hour
}

fn time_display(now: DateTime<Utc>, hour: i32, display_data: Vec<Vec<String>>) {
    let minute = now.minute();
    let digit_1 = (hour - (hour % 10)) / 10;
    let digit_2 = hour % 10;
    let digit_3 = (minute - (minute % 10)) / 10;
    let digit_4 = minute % 10;

    print!("{}", display_data[digit_1 as usize][0]);
    print!("{}", display_data[digit_2 as usize][0]);
    print!("  ");
    print!("{}", display_data[digit_3 as usize][0]);
    print!("{}", display_data[digit_4 as usize][0]);
    println!("  ");

    print!("{}", display_data[digit_1 as usize][1]);
    print!("{}", display_data[digit_2 as usize][1]);
    print!(" \u{2580}");
    print!("{}", display_data[digit_3 as usize][1]);
    print!("{}", display_data[digit_4 as usize][1]);
    println!("  ");

    print!("{}", display_data[digit_1 as usize][2]);
    print!("{}", display_data[digit_2 as usize][2]);
    print!(" \u{2580}");
    print!("{}", display_data[digit_3 as usize][2]);
    print!("{}", display_data[digit_4 as usize][2]);
    println!(" :{:02}", now.second());

    println!();
    println!(" Alarm set for --:--");
}
