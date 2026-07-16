use camino::Utf8PathBuf;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Default)]
pub struct CleanupStats {
    pub deleted_count: usize,
    pub bytes_saved: u64,
}

impl CleanupStats {
    /// Форматирует байты в человекочитаемый вид (B, KB, MB, GB)
    pub fn formatted_size(&self) -> String {
        let bytes = self.bytes_saved as f64;
        if bytes < 1024.0 {
            format!("{} B", bytes)
        } else if bytes < 1024.0 * 1024.0 {
            format!("{:.2} KB", bytes / 1024.0)
        } else if bytes < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.2} MB", bytes / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", bytes / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

pub struct Remover<'a> {
    paths: &'a [Utf8PathBuf],
}

impl<'a> Remover<'a> {
    pub fn new(paths: &'a [Utf8PathBuf]) -> Self {
        Self { paths }
    }

    /// Удаляет все файлы автоматически и возвращает статистику
    pub fn delete_all_auto(self) -> anyhow::Result<CleanupStats> {
        let mut stats = CleanupStats::default();
        let mut failed = 0;

        for path in self.paths {
            // Получаем размер файла ДО его удаления
            let file_size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);

            match fs::remove_file(path) {
                Ok(_) => {
                    println!("Deleted: {}", path);
                    stats.deleted_count += 1;
                    stats.bytes_saved += file_size;
                }
                Err(e) => {
                    eprintln!("Failed to delete {}: {}", path, e);
                    failed += 1;
                }
            }
        }

        if failed > 0 {
            eprintln!("Warning: failed to delete {} file(s).", failed);
        }

        Ok(stats)
    }

    /// Удаляет файлы в ручном режиме и возвращает статистику
    pub fn delete_manual(self) -> anyhow::Result<CleanupStats> {
        let mut stats = CleanupStats::default();
        let mut failed = 0;

        for path in self.paths {
            let prompt = format!("Delete unused image: {}", path);

            match ask_confirm(&prompt)? {
                true => {
                    let file_size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);

                    if let Err(e) = fs::remove_file(path) {
                        eprintln!("Failed to delete {}: {}", path, e);
                        failed += 1;
                    } else {
                        println!("Deleted: {}", path);
                        stats.deleted_count += 1;
                        stats.bytes_saved += file_size;
                    }
                }
                false => {
                    println!("Skipped: {}", path);
                }
            }
        }

        if failed > 0 {
            eprintln!(
                "Warning: manual deletion finished with {} error(s).",
                failed
            );
        }

        Ok(stats)
    }
}

pub fn ask_confirm(prompt: &str) -> io::Result<bool> {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_lowercase();

    Ok(trimmed == "y" || trimmed == "yes")
}
