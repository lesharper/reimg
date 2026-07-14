// scanner.rs
use camino::{Utf8Path, Utf8PathBuf};
use ignore::WalkBuilder;

use crate::filters;

#[derive(Debug)]
pub struct Paths {
    pub images: Vec<Utf8PathBuf>,
    pub others: Vec<Utf8PathBuf>,
}

pub struct Scanner<'a> {
    root: &'a Utf8Path,
}

impl<'a> Scanner<'a> {
    pub fn new(path: &'a Utf8Path) -> Self {
        Self { root: path }
    }

    /// Запускает сканирование и возвращает заполненную структуру Paths
    pub fn scan(self) -> Paths {
        let mut images = Vec::new();
        let mut others = Vec::new();

        let walker = WalkBuilder::new(self.root)
            .hidden(false)
            .git_ignore(true)
            .build();

        for entry in walker.filter_map(|e| e.ok()) {
            if let Some(path) = Utf8Path::from_path(entry.path()) {
                // Проверяем, что это файл, а не директория
                if path.is_file() {
                    // Клонируем путь только тогда, когда уверены, что он нам нужен
                    if filters::is_image(path) {
                        images.push(path.to_path_buf());
                    } else {
                        others.push(path.to_path_buf());
                    }
                }
            }
        }

        Paths { images, others }
    }
}
