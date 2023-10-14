use clap::Parser;
use drunken_bishop::World;
use sha256::digest;
use std::io;
use std::io::Read;

#[derive(Parser)]
#[clap(about, version)]
struct Args {
    /// Strings to visualize. All strings will be joined with a space. If none, read from stdin.
    #[clap()]
    string: Vec<String>,

    /// Use SHA-256 digest of given string
    #[clap(short, long)]
    sha256: bool,

    /// Verbose output
    #[clap(short, long)]
    verbose: bool,
}

pub fn main() {
    let Args {
        string,
        sha256,
        verbose,
    } = Args::parse();
    let mut input = get_input(&string);

    if verbose {
        println!("- Input string  : {input}");
    }

    if sha256 {
        input = digest(input);

        if verbose {
            println!("- SHA-256 digest: {input}");
        }
    }

    if verbose {
        println!("- Drunken bishop:");
    }

    println!("{}", World::from(&input))
}

fn get_input(arg: &[String]) -> String {
    let mut input = arg.join(" ");

    if input.trim().is_empty() {
        let mut stdin = String::new();
        io::stdin()
            .read_to_string(&mut stdin)
            .expect("Unable to read from stdin");
        input = stdin;
    }

    input
}
