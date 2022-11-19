use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Debug
    #[arg(short, long)]
    debug: bool,

    /// Write to file
    #[arg(short ='i', long)]
    file: bool,

    /// Write to db
    #[arg(short = 'b', long)]
    db: bool,

    /// Fetching from web
    #[arg(short = 'e', long)]
    fetch: bool,
}

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args = Args::parse();

    println!("{}", args.debug);
}
