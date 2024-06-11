use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use clap::{Arg, Command};
use notify_rust::Notification;
use std::{thread, time::Duration};

fn main() {
    // Parse command-line arguments
    let matches = Command::new("Reminder App")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Sends a notification at a specified time")
        .arg(
            Arg::new("message")
                .short('m')
                .long("message")
                .value_name("MESSAGE")
                .help("The reminder message")
                .required(true),
        )
        .arg(
            Arg::new("time")
                .short('t')
                .long("time")
                .value_name("TIME")
                .help("The reminder time (YYYY-MM-DD HH:MM:SS)")
                .required(true),
        )
        .get_matches();

    let reminder_message = matches.get_one::<String>("message").unwrap();
    let reminder_time_str = matches.get_one::<String>("time").unwrap();

    // Parse the reminder time
    let reminder_time = match NaiveDateTime::parse_from_str(reminder_time_str, "%Y-%m-%d %H:%M:%S") {
        Ok(parsed_time) => parsed_time,
        Err(_) => {
            eprintln!("Failed to parse the reminder time. Please use the format YYYY-MM-DD HH:MM:SS");
            return;
        }
    };

    // Convert to a DateTime<Local>
    let reminder_time: DateTime<Local> = Local.from_local_datetime(&reminder_time).unwrap();

    // Calculate the duration to wait
    let now = Local::now();
    let duration_to_wait = reminder_time - now;

    if duration_to_wait.num_seconds() <= 0 {
        eprintln!("The specified time is in the past.");
        return;
    }

    println!("Reminder set for: {}", reminder_time);

    // Sleep until the specified time
    thread::sleep(Duration::from_secs(duration_to_wait.num_seconds() as u64));

    // Send the notification
    Notification::new()
        .summary("Reminder")
        .body(reminder_message)
        .show()
        .unwrap();
}
