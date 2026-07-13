use camino::{Utf8Path, Utf8PathBuf};
use ignore::WalkBuilder;

use crate::filters;

pub struct Scanner<'a> {
    root: &'a Utf8Path,
    pub paths: Vec<Utf8PathBuf>,
}

impl<'a> Scanner<'a> {
    pub fn new(path: &'a Utf8Path) -> Self {
        Self {
            root: path,
            paths: Vec::new(),
        }
    }

    pub fn get_paths(&mut self) {
        self.paths = WalkBuilder::new(self.root)
            .hidden(false)
            .git_ignore(true)
            .build()
            // filter_map позволяет элегантно отсеять ошибки и распаковать Ok
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                // Конвертируем стандартный Path в Utf8Path без аллокаций (если валидный UTF-8)
                let path = Utf8Path::from_path(e.path())?;

                // Проверяем, что это файл (не директория .png, например) и что это картинка
                if path.is_file() && filters::is_image(path) {
                    // Если нужен абсолютный путь, лучше резолвить его один раз тут
                    // Но учти, что для поиска по коду тебе скорее понадобятся относительные пути
                    Some(path.to_path_buf())
                } else {
                    None
                }
            })
            .collect();
    }
}
