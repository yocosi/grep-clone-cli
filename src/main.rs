use anyhow::{Context, Result};
use clap::Parser;
//use std::any::type_name;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

//fn type_of<T>(_: T) -> &'static str {
//    type_name::<T>()
//}

// To check in the path if there is a match of the pattern
fn find_a_match(f: &File, pattern: &String, cpt: &mut i32) {
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let l = line.unwrap(); // To be able to use the .contains method
        if l.contains(pattern) {
            println!("{}", l);
            *cpt += 1;
        }
    }
}

fn main() -> Result<()> {
    let mut cpt = 0;
    let args = Cli::parse();

    // ? at the end to be able to handle errors
    let f = File::open(&args.path)
        .with_context(|| format!("could not read file`{}`", &args.path.display()))?;

    find_a_match(&f, &args.pattern, &mut cpt);

    if cpt == 0 {
        println!("No match found.");
    } else {
        println!("{} match found!", cpt);
    }

    Ok(())
}
