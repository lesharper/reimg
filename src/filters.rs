use camino::Utf8Path;

pub fn is_image(path: &Utf8Path) -> bool {
    let Some(ext) = path.extension() else {
        return false;
    };

    matches!(
        ext,
        e if e.eq_ignore_ascii_case("jpg")
            || e.eq_ignore_ascii_case("jpeg")
            || e.eq_ignore_ascii_case("png")
            //|| e.eq_ignore_ascii_case("gif")
            //|| e.eq_ignore_ascii_case("svg")
            || e.eq_ignore_ascii_case("webp")
            || e.eq_ignore_ascii_case("avif")
    )
}
