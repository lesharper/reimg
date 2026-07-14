// analyzer.rs
use aho_corasick::{AhoCorasick, MatchKind};
use camino::Utf8PathBuf;
use std::fs;

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

        // Вектор флагов: true означает, что картинка используется в коде
        let mut matched = vec![false; self.images.len()];
        let mut matched_count = 0;
        let total_images = self.images.len();

        for file_path in files {
            // Читаем файл как строку. Если это бинарник или системный мусор — read_to_string
            // вернет ошибку, и мы просто пойдем дальше, что абсолютно безопасно.
            let Ok(content) = fs::read_to_string(file_path) else {
                continue;
            };

            // Сканируем контент автомата за один проход
            for mat in self.ac.find_iter(&content) {
                let idx = mat.pattern().as_usize();

                if !matched[idx] {
                    matched[idx] = true;
                    matched_count += 1;

                    // Оптимизация "Ранний выход": если мы уже нашли упоминание абсолютно
                    // ВСЕХ картинок проекта, продолжать читать оставшиеся файлы бессмысленно.
                    if matched_count == total_images {
                        return Vec::new();
                    }
                }
            }
        }

        // Собираем пути картинок, у которых флаг остался false
        self.images
            .iter()
            .enumerate()
            .filter(|(idx, _)| !matched[*idx])
            .map(|(_, path)| path.clone())
            .collect()
    }
}
