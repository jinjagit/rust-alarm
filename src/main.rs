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
        let hour:u32 = correct_hour_with_offset(now.hour(), offset);
    
        stdout.execute(cursor::MoveTo(0,0)).ok();

        time_display(now, hour, display_data.clone());

        thread::sleep(one_second);
    }
}

fn correct_hour_with_offset(hour: u32, offset: i32) -> u32 {
    let mut corrected = hour as i32 + offset;

    if corrected > 24 {
        corrected = corrected - 24;
    }

    if corrected < 0 {
        corrected = 24 - corrected;
    }

    corrected as u32
}

fn time_display(now: DateTime<Utc>, hour: u32, display_data: Vec<Vec<String>>) {
    let minute = now.minute();
    let digit_1 = (hour - (hour % 10)) / 10;
    let digit_2 = hour % 10;
    let digit_3 = (minute - (minute % 10)) / 10;
    let digit_4 = minute % 10;
    let mut stdout = stdout();

    print!("{}", display_data[digit_1 as usize][0]);
    print!("{}", display_data[digit_2 as usize][0]);
    print!("  ");
    print!("{}", display_data[digit_3 as usize][0]);
    print!("{}", display_data[digit_4 as usize][0]);
    println!("        ");

    stdout.execute(cursor::MoveTo(0,1)).ok();
    print!("{}", display_data[digit_1 as usize][1]);
    print!("{}", display_data[digit_2 as usize][1]);
    print!(" \u{2580}");
    print!("{}", display_data[digit_3 as usize][1]);
    print!("{}", display_data[digit_4 as usize][1]);
    println!("        ");

    stdout.execute(cursor::MoveTo(0,2)).ok();
    print!("{}", display_data[digit_1 as usize][2]);
    print!("{}", display_data[digit_2 as usize][2]);
    print!(" \u{2580}");
    print!("{}", display_data[digit_3 as usize][2]);
    print!("{}", display_data[digit_4 as usize][2]);
    print!(" :{:02}", now.second());
    println!("        ");

    stdout.execute(cursor::MoveTo(0,3)).ok();
    println!();

    stdout.execute(cursor::MoveTo(0,4)).ok();
    println!(" Alarm set for --:--");
}
