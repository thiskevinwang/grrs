use chrono::{DateTime, Duration, Local, Timelike};
use crossbeam_channel::{bounded, select, tick, Receiver};
use mac_notification_sys::*;
use ms::*;
use std::io;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);

    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

#[tokio::main]
async fn main() -> Result<(), exitfailure::ExitFailure> {
    let duration: i64;

    // ***************
    // READ USER INPUT
    // ***************
    println!("How long do you want to set a timer for?");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();

    duration = ms!(trimmed).unwrap() as i64;

    // *******************
    // USER INPUT FEEDBACK
    // *******************
    println!("Setting a timer for {}", ms!(duration as u64, true));

    let start_time: DateTime<Local> = Local::now();
    let (is_pm, hour) = start_time.hour12();

    let start_time_formatted = format!(
        "{:02}:{:02}:{:02} {}",
        hour,
        start_time.minute(),
        start_time.second(),
        if is_pm { "PM" } else { "AM" }
    );
    let end_time = start_time + Duration::milliseconds(duration);
    let (is_pm, hour) = end_time.hour12();

    let end_time_formatted = format!(
        "{:02}:{:02}:{:02} {}",
        hour,
        end_time.minute(),
        end_time.second(),
        if is_pm { "PM" } else { "AM" }
    );

    // **************
    // DISPLAYS BUNDLE ICON
    // calendar
    // steam
    // **************
    let bundle = get_bundle_identifier_or_default("notes");
    set_application(&bundle).unwrap();

    // ********************
    // INITIAL NOTIFICATION
    // ********************
    send_notification(
        &format!("Timer set for: {}", ms!(duration as u64, true)),
        &Some(&format!("⏰ {}", start_time_formatted)),
        &format!("🛑 {}", end_time_formatted),
        &Some("Morse"),
    )
    .unwrap();

    let ctrl_c_events = ctrl_channel()?;

    // *********
    // frequency
    // *********
    let ticks = tick(tokio::time::Duration::from_millis((duration / 100) as u64));

    // ***********
    // loop & tick
    // ***********
    loop {
        select! {
            recv(ticks) -> _ => {
                let now: DateTime<Local> = Local::now();
                let elapsed = now.timestamp() - start_time.timestamp();
                let total = end_time.timestamp() - start_time.timestamp();
                let percent = elapsed as f64 / total as f64;
                // ************
                // exit if 100%
                // ************
                if percent >= 1f64 {
                    break
                };

                send_notification(
                    &format!("Progress: {:.2}%", percent * 100f64),
                    &Some(&format!("⏰ {}", start_time_formatted)),
                    &format!("🛑 {}", end_time_formatted),
                    &Some("Morse")
                ).unwrap();
            }
            recv(ctrl_c_events) -> _=> {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    // **********************
    // SEND DONE NOTIFICATION
    // **********************
    send_notification(
        &"Done!",
        &Some(&format!("⏰ {}", start_time_formatted)),
        &format!("🛑 {}", end_time_formatted),
        &Some("Morse"),
    )
    .unwrap();
    Ok(())
}
