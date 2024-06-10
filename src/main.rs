use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use notify_rust::Notification;
use std::{thread, time::Duration};

fn main() {
    // Prompt the user for the reminder message and time
    let reminder_message = prompt("Enter the reminder message:");
    let reminder_time_str = prompt("Enter the reminder time (YYYY-MM-DD HH:MM:SS):");

    // Parse the reminder time
    let reminder_time = match NaiveDateTime::parse_from_str(&reminder_time_str, "%Y-%m-%d %H:%M:%S") {
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
        .body(&reminder_message)
        .show()
        .unwrap();
}

fn prompt(message: &str) -> String {
    use std::io::{self, Write};

    print!("{} ", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
