use chrono::{DateTime, Duration, Local, Timelike};
use crossbeam_channel::{bounded, select, tick, Receiver};
use mac_notification_sys::*;
// use serde_json::json;
// use tokio::time::{self, Duration};

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);

    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

#[tokio::main]
async fn main() -> Result<(), exitfailure::ExitFailure> {
    let duration = 60 * 30;

    let start_time: DateTime<Local> = Local::now();
    let (is_pm, hour) = start_time.hour12();

    let start_time_formatted = format!(
        "{:02}:{:02}:{:02} {}",
        hour,
        start_time.minute(),
        start_time.second(),
        if is_pm { "PM" } else { "AM" }
    );
    let end_time = start_time + Duration::seconds(duration);
    let (is_pm, hour) = end_time.hour12();

    let end_time_formatted = format!(
        "{:02}:{:02}:{:02} {}",
        hour,
        end_time.minute(),
        end_time.second(),
        if is_pm { "PM" } else { "AM" }
    );

    // calendar
    // steam
    //
    let bundle = get_bundle_identifier_or_default("steam");
    set_application(&bundle).unwrap();

    // let mut interval = time::interval(Duration::seconds(5));
    // interval.tick().await;
    send_notification(
        &format!("Timer set for: {}s", duration),
        &Some(&format!("⏰ {}", start_time_formatted)),
        &format!("🛑 {}", end_time_formatted),
        &Some("Morse"),
    )
    .unwrap();

    let ctrl_c_events = ctrl_channel()?;

    // frequency
    let ticks = tick(tokio::time::Duration::from_secs((duration / 100) as u64));

    loop {
        select! {
            recv(ticks) -> _ => {
                let now: DateTime<Local> = Local::now();
                let elapsed = now.timestamp() - start_time.timestamp();
                let total = end_time.timestamp() - start_time.timestamp();
                let percent = elapsed as f64 / total as f64;
                if percent >= 1f64 {
                    break
                };
                // println!("{:.2}", percent);
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

    send_notification(
        &"Done!",
        &Some(&format!("⏰ {}", start_time_formatted)),
        &format!("🛑 {}", end_time_formatted),
        &Some("Morse"),
    )
    .unwrap();
    Ok(())
}
