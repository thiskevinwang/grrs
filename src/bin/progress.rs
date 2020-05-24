use exitfailure::ExitFailure;
use failure::ResultExt;
use indicatif::ProgressBar;

/// run with
/// `cargo run --bin progress`
///
/// https://rust-cli.github.io/book/tutorial/output.html#showing-a-progress-bar
///
fn main() {
    let pb = ProgressBar::new(100);
    for i in 0..100 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("DONE!")
}
