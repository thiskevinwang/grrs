use signal_hook::{iterator::Signals, SIGINT};
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for signal in signals.forever() {
            // Ctrl+C == 2
            println!("Received signal {:?}", signal);
        }
    });

    thread::sleep(Duration::from_secs(2));

    Ok(())
}
