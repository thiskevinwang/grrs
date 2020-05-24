use serde_json::json;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long = "json")]
    json: bool,
}

fn main() {
    let args = Cli::from_args();

    if args.json
    // cargo run --bin json_output -- --json
    {
        println!(
            "{}",
            json!({
                "type": "message",
                "content": "Hello world",
            })
        )
    } else
    // cargo run --bin json_output
    {
        println!("Hello world")
    }
}
