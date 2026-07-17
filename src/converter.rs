use crate::cli::{Format, Mode};
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use image::ImageReader;
use rayon::prelude::*;
use rgb::FromSlice;
use std::fs;

pub struct Converter;

impl Converter {
    pub fn new() -> Self {
        Self {}
    }

    /// Возвращает вектор сопоставлений: (Старый путь, Новый путь)
    pub fn process(
        &self,
        paths: &[Utf8PathBuf],
        mode: Mode,
    ) -> Result<Vec<(Utf8PathBuf, Utf8PathBuf)>> {
        match mode {
            Mode::Manual => self.process_manual(paths),
            Mode::Auto => self.process_auto(paths),
        }
    }

    /// Однопоточный ручной режим (спрашиваем каждый файл)
    fn process_manual(&self, paths: &[Utf8PathBuf]) -> Result<Vec<(Utf8PathBuf, Utf8PathBuf)>> {
        let mut mapped = Vec::new();
        for path in paths {
            let prompt = format!("Convert '{}'?", path.file_name().unwrap_or(""));
            if let Some(format) = crate::cli::ask_format(&prompt)? {
                match self.convert_single(path, format) {
                    Ok(new_path) => {
                        println!("Success: {}", new_path);
                        mapped.push((path.clone(), new_path));
                    }
                    Err(e) => eprintln!("Failed to convert {}: {}", path, e),
                }
            } else {
                println!("Skipped: {}", path);
            }
        }
        Ok(mapped)
    }

    /// Многопоточный автоматический режим
    fn process_auto(&self, paths: &[Utf8PathBuf]) -> Result<Vec<(Utf8PathBuf, Utf8PathBuf)>> {
        let format = loop {
            if let Some(f) = crate::cli::ask_format("Select target format for ALL files")? {
                break f;
            }
            println!("In Auto mode you must select a valid format ('webp' or 'avif').");
        };

        println!(
            "\nConverting {} files to {:?} (Multi-threaded)...",
            paths.len(),
            format
        );

        // Магия Rayon (par_iter): обрабатываем картинки параллельно
        let mapped: Vec<_> = paths
            .par_iter()
            .filter_map(|path| match self.convert_single(path, format) {
                Ok(new_path) => {
                    println!("Converted: {}", new_path.file_name().unwrap());
                    Some((path.clone(), new_path))
                }
                Err(e) => {
                    eprintln!("Error converting {}: {}", path, e);
                    None
                }
            })
            .collect();

        Ok(mapped)
    }

    /// Основная логика: сжатие и замена на диске
    fn convert_single(&self, path: &Utf8PathBuf, format: Format) -> Result<Utf8PathBuf> {
        // Декодируем исходник
        let img = ImageReader::open(path)
            .with_context(|| format!("Could not open {}", path))?
            .decode()
            .with_context(|| format!("Could not decode {}", path))?;

        let new_ext = match format {
            Format::Webp => "webp",
            Format::Avif => "avif",
        };
        let new_path = path.with_extension(new_ext);

        match format {
            Format::Webp => {
                let encoder = webp::Encoder::from_image(&img)
                    .map_err(|_| anyhow::anyhow!("Failed to init WebP encoder"))?;
                // 80.0 - качество. Можно вынести в настройки CLI
                let memory = encoder.encode(80.0);
                fs::write(&new_path, &*memory)?;
            }
            Format::Avif => {
                let rgba = img.to_rgba8();
                let width = rgba.width() as usize;
                let height = rgba.height() as usize;

                // as_rgba() берет &[u8] и безопасно кастует его в &[Rgba<u8>]
                let pixels = rgba.as_raw().as_rgba();

                let ravif_img = ravif::Img::new(pixels, width, height);
                let encoder = ravif::Encoder::new().with_quality(80.0).with_speed(4);

                let result = encoder
                    .encode_rgba(ravif_img)
                    .map_err(|e| anyhow::anyhow!("AVIF error: {:?}", e))?;

                fs::write(&new_path, result.avif_file)?;
            }
        }

        // Удаляем оригинал после успешной записи
        fs::remove_file(path)?;

        Ok(new_path)
    }
}
