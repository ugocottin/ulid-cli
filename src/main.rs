use clap::Parser;
use ulid::Ulid;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::new())]
    prefix: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        let ulid = Ulid::new();
        if !args.prefix.is_empty() {
            print!("{}_", args.prefix);
        }
        println!("{}", ulid.to_string());
    }
}
