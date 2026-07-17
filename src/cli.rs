use camino::Utf8PathBuf;
use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(name = "reimg", version, about = "Image search and processing")]
pub struct Cli {
    #[arg(value_name = "PATH")]
    pub path: Utf8PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Auto,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Webp,
    Avif,
}

/// Базовый вопрос (Да/Нет)
pub fn ask_confirm(prompt: &str) -> io::Result<bool> {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_lowercase();
    Ok(trimmed == "y" || trimmed == "yes")
}

/// Спрашивает режим работы
pub fn ask_mode(prompt: &str) -> io::Result<Mode> {
    loop {
        print!("{} [auto/manual]: ", prompt);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "auto" | "a" => return Ok(Mode::Auto),
            "manual" | "m" => return Ok(Mode::Manual),
            _ => println!("Invalid input. Please enter 'auto' or 'manual'."),
        }
    }
}

/// Спрашивает формат для конвертации
pub fn ask_format(prompt: &str) -> io::Result<Option<Format>> {
    loop {
        print!("{} [webp/avif/skip]: ", prompt);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "webp" | "w" => return Ok(Some(Format::Webp)),
            "avif" | "a" => return Ok(Some(Format::Avif)),
            "skip" | "s" | "n" | "no" => return Ok(None),
            _ => println!("Invalid input. Please enter 'webp', 'avif' or 'skip'."),
        }
    }
}
