// main.rs
mod cli;
mod filters;
mod scanner;

use anyhow::Result;
use clap::Parser;
use std::time::Instant;

fn main() -> Result<()> {
    let start = Instant::now();
    let args = cli::Cli::parse();

    if !args.path.exists() || !args.path.is_dir() {
        anyhow::bail!("Path does not exist or is not a directory: {}", args.path);
    }

    // Просто вызываем цепочкой. Scanner отработал и очистил память, остались только пути.
    let founded_paths = scanner::Scanner::new(&args.path).scan();

    println!("Found images: {}", founded_paths.images.len());
    println!("Found other files: {}", founded_paths.others.len());

    let end = start.elapsed();
    println!("Total execution time: {:.2?}", end);

    Ok(())
}
