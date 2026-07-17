use aho_corasick::AhoCorasick;
use camino::Utf8PathBuf;
use rayon::prelude::*;
use std::fs;

pub fn update_code_references(
    files: &[Utf8PathBuf],
    mappings: &[(Utf8PathBuf, Utf8PathBuf)],
) -> anyhow::Result<()> {
    if mappings.is_empty() {
        return Ok(());
    }

    let mut patterns = Vec::new();
    let mut replacements = Vec::new();

    // Собираем массив имен: "logo.png" -> "logo.webp"
    for (old_path, new_path) in mappings {
        if let (Some(old_name), Some(new_name)) = (old_path.file_name(), new_path.file_name()) {
            patterns.push(old_name.to_string());
            replacements.push(new_name.to_string());
        }
    }

    let ac = AhoCorasick::new(&patterns)?;

    // Параллельная перезапись ссылок в коде проекта
    files.par_iter().for_each(|file_path| {
        if let Ok(content) = fs::read_to_string(file_path) {
            if ac.is_match(&content) {
                let new_content = ac.replace_all(&content, &replacements);
                // Игнорируем ошибки записи, чтобы не ронять весь пайплайн
                let _ = fs::write(file_path, new_content);
            }
        }
    });

    Ok(())
}
