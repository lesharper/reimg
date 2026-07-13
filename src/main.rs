mod cli;
mod filters;
mod scanner;

use anyhow::Result;
use clap::Parser;

use std::time::Instant;

fn main() -> Result<()> {
    let start = Instant::now();

    let args = cli::Cli::parse();

    if !args.path.exists() {
        anyhow::bail!("Path does not exist: {}", args.path);
    }
    if !args.path.is_dir() {
        anyhow::bail!("Path is not a directory: {}", args.path);
    }

    let mut scanner = scanner::Scanner::new(&args.path);

    scanner.get_paths();

    let founded_images = scanner.paths;

    dbg!(founded_images);

    let end = start.elapsed();
    println!("Total execution time: {:.2?}", end);

    Ok(())
}
