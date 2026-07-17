// analyzer.rs
use aho_corasick::{AhoCorasick, MatchKind};
use camino::Utf8PathBuf;
use rayon::prelude::*;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Analyzer {
    images: Vec<Utf8PathBuf>,
    ac: AhoCorasick,
}

impl Analyzer {
    /// Инициализирует анализатор и строит поисковый автомат
    pub fn new(images: Vec<Utf8PathBuf>) -> anyhow::Result<Self> {
        // Извлекаем только имена файлов (например, "logo.png"), так как именно
        // в таком виде они чаще всего встречаются внутри кода (src="./logo.png")
        let patterns: Vec<String> = images
            .iter()
            .filter_map(|p| p.file_name().map(|name| name.to_string()))
            .collect();

        // Строим автомат. Режим LeftmostFirst идеален для поиска точных совпадений подстрок.
        let ac = AhoCorasick::builder()
            .match_kind(MatchKind::LeftmostFirst)
            .build(&patterns)?;

        Ok(Self { images, ac })
    }

    /// Проверяет текстовые файлы проекта и возвращает список НЕиспользуемых изображений
    pub fn find_unused(&self, files: &[Utf8PathBuf]) -> Vec<Utf8PathBuf> {
        if self.images.is_empty() {
            return Vec::new();
        }

        // Атомарные флаги для безопасной записи из разных потоков
        let matched: Vec<AtomicBool> = (0..self.images.len())
            .map(|_| AtomicBool::new(false))
            .collect();

        // Параллельный итератор Rayon
        files.par_iter().for_each(|file_path| {
            if let Ok(content) = fs::read_to_string(file_path) {
                for mat in self.ac.find_iter(&content) {
                    matched[mat.pattern().as_usize()].store(true, Ordering::Relaxed);
                }
            }
        });

        self.images
            .iter()
            .enumerate()
            .filter(|(idx, _)| !matched[*idx].load(Ordering::Relaxed))
            .map(|(_, path)| path.clone())
            .collect()
    }
}
