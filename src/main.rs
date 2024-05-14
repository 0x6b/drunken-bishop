use std::{io, io::Read};

use clap::Parser;
use drunken_bishop::World;
use log::debug;
use sha256::digest;
use tracing::Level;

#[derive(Parser)]
#[clap(about, version)]
struct Args {
    /// Strings to visualize. All strings will be joined with a space. If none, read from stdin.
    #[arg()]
    string: Vec<String>,

    /// Use SHA-256 digest of given string
    #[arg(short, long)]
    sha256: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

pub fn main() {
    let Args { string, sha256, verbose } = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(if verbose { Level::DEBUG } else { Level::INFO })
        .init();

    let mut input = get_input(&string);

    debug!("Input string: {input}");
    debug!("SHA-256: {sha256}");
    debug!("Verbose: {verbose}");

    if sha256 {
        input = digest(input);
        debug!("SHA-256 digest: {input}");
    }

    println!("{}", World::from(&input))
}

fn get_input(args: &[String]) -> String {
    let input = args.join(" ");

    if input.trim().is_empty() {
        let mut stdin = String::new();
        io::stdin()
            .read_to_string(&mut stdin)
            .expect("Unable to read from stdin");
        stdin
    } else {
        input
    }
}
