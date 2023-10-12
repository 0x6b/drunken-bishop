use clap::Parser;
use drunken_bishop::World;
use sha256::digest;

#[derive(Parser)]
#[clap(about, version)]
struct Args {
    /// Strings to visualize. All strings will be joined with a space.
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
    let string = string.join(" ");

    if verbose {
        println!("- Input string  : {string}");
    }

    let string = if sha256 { digest(string) } else { string };

    if verbose && sha256 {
        println!("- SHA-256 digest: {string}");
    }

    if verbose {
        println!("- Drunken bishop:");
    }

    println!("{}", World::from(&string))
}
