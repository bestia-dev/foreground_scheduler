//! scheduler_mod

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
use chrono::{Local, Timelike, Utc};
use std::io::{self, Write};
//use unwrap::unwrap;

/// loop and sleep scheduler
pub fn loop_scheduler(minute: usize, command: &str, args: &str) {
    loop {
        if is_scheduled_run(minute) {
            println!("{}", "");
            println!("utc  : {}", &Utc::now().format("%Y-%m-%d %H:%M:%S"));
            println!("local: {}", &Local::now().format("%Y-%m-%d %H:%M:%S"));
            println!("Executed every hour on {} minute utc.", minute);
            run_command(command, &args);
        }
        // I need the tick resolution once per minute
        std::thread::sleep(std::time::Duration::from_millis(millis_until_next_minute()));
        if false {
            break;
        }
    }
}

/// run command
fn run_command(command: &str, args: &str) {
    use std::process::Command;
    println!(" $ {} {}", command, args);
    let args: Vec<&str> = args.split(' ').collect();
    Command::new(command)
        .args(args)
        .spawn()
        .expect("command failed to start");
}

/// if the time is exactly as scheduled, then return true
/// resolution is 1 minute.
pub fn is_scheduled_run(minute: usize) -> bool {
    let now = Utc::now();
    let now_minute = now.minute();
    if now_minute == minute as u32 {
        return true;
    } else {
        print!("{}...", now_minute);
        io::stdout().flush().unwrap();
        return false;
    }
}

/// millis until next minute. I use this to sleep.
pub fn millis_until_next_minute() -> u64 {
    let now = Utc::now();
    //return
    (60u64 - now.second() as u64) * 1000u64
}
