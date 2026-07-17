mod analyzer;
mod cli;
mod converter;
mod filters;
mod remover;
mod replacer;
mod scanner;

use anyhow::Result;
use clap::Parser;
use std::time::Instant;

fn run_pipeline() -> Result<()> {
    let args = cli::Cli::parse();

    if !args.path.exists() || !args.path.is_dir() {
        anyhow::bail!("Path does not exist or is not a directory: {}", args.path);
    }

    println!("Scanning project structure...");
    let scan_res = scanner::Scanner::new(&args.path).scan();
    println!(
        "Found {} images and {} text files.",
        scan_res.images.len(),
        scan_res.others.len()
    );

    println!("\nAnalyzing image usage...");
    let mut active_images = scan_res.images.clone();
    let analyzer = analyzer::Analyzer::new(scan_res.images)?;
    let unused_images = analyzer.find_unused(&scan_res.others);

    // БЛОК 1: ОЧИСТКА
    if !unused_images.is_empty() {
        println!("Found {} unused image(s).", unused_images.len());

        if cli::ask_confirm("Clean up unused images?")? {
            let auto = cli::ask_confirm("Delete ALL automatically?")?;
            let remover = remover::Remover::new(&unused_images);

            let stats = if auto {
                remover.delete_all_auto()?
            } else {
                remover.delete_manual()?
            };

            println!(
                "\nCleanup: {} files deleted, {} reclaimed.",
                stats.deleted_count,
                stats.formatted_size()
            );

            // Очищаем вектор от удаленных, чтобы не пытаться их конвертировать
            active_images.retain(|img| !unused_images.contains(img));
        }
    } else {
        println!("All images are actively used!");
    }

    // БЛОК 2: КОНВЕРТАЦИЯ И ЗАМЕНА
    if !active_images.is_empty()
        && cli::ask_confirm("\nDo you want to convert/compress the remaining images?")?
    {
        let mode = cli::ask_mode("Choose conversion mode")?;
        let converter = converter::Converter::new();

        let mappings = converter.process(&active_images, mode)?;

        if !mappings.is_empty() {
            println!("\nUpdating file references in project code...");
            replacer::update_code_references(&scan_res.others, &mappings)?;
            println!("✅ Code updated successfully.");
        }
    }

    Ok(())
}

fn main() {
    let start = Instant::now();

    if let Err(e) = run_pipeline() {
        eprintln!("Fatal error: {:?}", e);
        std::process::exit(1);
    }

    println!("\nTotal execution time: {:.2?}", start.elapsed());
}
