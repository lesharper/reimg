// remover.rs
use camino::Utf8PathBuf;
use std::fs;
use std::io::{self, Write};

pub struct Remover<'a> {
    // Используем срез &[Utf8PathBuf] вместо ссылки на Vec
    paths: &'a [Utf8PathBuf],
}

impl<'a> Remover<'a> {
    pub fn new(paths: &'a [Utf8PathBuf]) -> Self {
        Self { paths }
    }

    /// Удаляет все файлы автоматически.
    /// Вместо unwrap() собирает ошибки, чтобы один сбойный файл не прерывал удаление остальных.
    pub fn delete_all_auto(self) -> anyhow::Result<()> {
        let mut failed = 0;

        for path in self.paths {
            match fs::remove_file(path) {
                Ok(_) => println!("Deleted: {}", path),
                Err(e) => {
                    eprintln!("Failed to delete {}: {}", path, e);
                    failed += 1;
                }
            }
        }

        if failed > 0 {
            anyhow::bail!("Finished with errors: failed to delete {} file(s).", failed);
        }

        Ok(())
    }

    /// Пофайловое удаление с подтверждением от пользователя (Manual mode)
    pub fn delete_manual(self) -> anyhow::Result<()> {
        let mut failed = 0;

        for path in self.paths {
            let prompt = format!("Delete unused image: {}", path);

            match ask_confirm(&prompt)? {
                true => {
                    if let Err(e) = fs::remove_file(path) {
                        eprintln!("Failed to delete {}: {}", path, e);
                        failed += 1;
                    } else {
                        println!("Deleted: {}", path);
                    }
                }
                false => {
                    println!("Skipped: {}", path);
                }
            }
        }

        if failed > 0 {
            anyhow::bail!("Manual deletion finished with {} error(s).", failed);
        }

        Ok(())
    }
}

/// Утилитарная функция для получения подтверждения [y/N] через консоль
pub fn ask_confirm(prompt: &str) -> io::Result<bool> {
    print!("{} [y/N]: ", prompt);
    // Нам нужно принудительно сбросить буфер stdout,
    // чтобы текст вывелся до того, как программа заблокируется на чтении ввода
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_lowercase();

    Ok(trimmed == "y" || trimmed == "yes")
}
