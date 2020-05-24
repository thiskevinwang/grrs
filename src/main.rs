use std::io::{self, Write};

/// Run with:
/// `cargo run`
///
/// ⚠️ If you have a `/src/bin` directory, cargo run will
/// search for a binary to run there, and ignore `/src/main.rs`
///
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it
    writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            return Err(error.into());
        }
    };
    println!("file content: {}", content);
    Ok(())
}
