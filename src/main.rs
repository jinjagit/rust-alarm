mod digits_data;
use digits_data::digits_data;
use chrono::{Timelike, Utc};
use std::{io::stdout, thread, time};
use crossterm::{ExecutableCommand, cursor};

struct AlarmData {
    active: bool,
    hour: u32,
    min: u32
}

fn main() {
    let mut stdout = stdout();
    let digits_data = digits_data();
    let offset: i32 = 1;
    let half_a_second = time::Duration::from_millis(500);
    let mut tick = 0;

    let mut alarm = AlarmData {
        active: false,
        hour: 19,
        min: 42
    };

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear terminal
    stdout.execute(cursor::Hide).ok();

    loop {
        let now = Utc::now();
        let hour:u32 = correct_hour_with_offset(now.hour(), offset);
        let minute = now.minute();
        let second = now.second();

        if tick == 2 { tick = 0; }

        if alarm.hour == hour && alarm.min == minute { alarm.active = true; }

        time_display(hour, minute, second, &alarm, digits_data.clone(), tick);

        tick += 1;

        thread::sleep(half_a_second);
    }
}

fn correct_hour_with_offset(hour: u32, offset: i32) -> u32 {
    let mut corrected = hour as i32 + offset;

    if corrected >= 24 { corrected = corrected - 24; }
    if corrected < 0 { corrected = 24 - corrected; }

    corrected as u32
}

fn time_display(hour: u32, minute: u32, second: u32, alarm: &AlarmData, digits_data: Vec<Vec<String>>, tick: i32) {
    let digit_1 = (hour - (hour % 10)) / 10;
    let digit_2 = hour % 10;
    let digit_3 = (minute - (minute % 10)) / 10;
    let digit_4 = minute % 10;
    let mut stdout = stdout();

    if alarm.active == false || tick == 0 {
        stdout.execute(cursor::MoveTo(0,0)).ok();
        print!("{}", digits_data[digit_1 as usize][0]);
        print!("{}", digits_data[digit_2 as usize][0]);
        print!("  ");
        print!("{}", digits_data[digit_3 as usize][0]);
        print!("{}", digits_data[digit_4 as usize][0]);
        println!("        ");

        stdout.execute(cursor::MoveTo(0,1)).ok();
        print!("{}", digits_data[digit_1 as usize][1]);
        print!("{}", digits_data[digit_2 as usize][1]);
        print!(" \u{2580}");
        print!("{}", digits_data[digit_3 as usize][1]);
        print!("{}", digits_data[digit_4 as usize][1]);
        println!("        ");

        stdout.execute(cursor::MoveTo(0,2)).ok();
        print!("{}", digits_data[digit_1 as usize][2]);
        print!("{}", digits_data[digit_2 as usize][2]);
        print!(" \u{2580}");
        print!("{}", digits_data[digit_3 as usize][2]);
        print!("{}", digits_data[digit_4 as usize][2]);
        if tick == 0 {
            print!(" :{:02}", second);
        } else {
            print!("  {:02}", second);
        }
        println!("        ");
    } else {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear terminal
    }

    stdout.execute(cursor::MoveTo(0,3)).ok();
    println!();

    stdout.execute(cursor::MoveTo(0,4)).ok();
    print!(" \u{25F7} Alarm set for ");
    println!("{:02}:{:02}", alarm.hour, alarm.min);
    // println!(" snooze:spc, reset:rtn");
}
