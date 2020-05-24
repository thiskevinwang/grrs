use env_logger;
use log::{info, warn};

/// run with
/// `cargo run --bin output-log`
/// - or -
/// `env RUST_LOG=output_log=info cargo run --bin output-log`
fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}
