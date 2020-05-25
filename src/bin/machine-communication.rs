use atty::Stream;

// TTY stands for [T]ele[TY]pewriter
fn main() {
    if atty::is(Stream::Stdout) {
        println!("I'm a terminal");
    } else {
        println!("I'm not")
    }
}
