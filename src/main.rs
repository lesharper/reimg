mod analyzer;
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

    println!("Scanning project structure...");
    let scan_res = scanner::Scanner::new(&args.path).scan();
    println!("Found images: {}", scan_res.images.len());
    println!(
        "Found code/text files to analyze: {}",
        scan_res.others.len()
    );

    println!("\nAnalyzing image usage via Aho-Corasick...");
    let analyzer = analyzer::Analyzer::new(scan_res.images)?;
    let unused_images = analyzer.find_unused(&scan_res.others);

    println!("\n--- Analysis Results ---");
    if unused_images.is_empty() {
        println!("All images are actively used in the project!");
    } else {
        println!("Found {} unused image(s):", unused_images.len());
        for img in &unused_images {
            println!("  - {}", img);
        }
    }

    let end = start.elapsed();
    println!("\nTotal execution time: {:.2?}", end);

    Ok(())
}
