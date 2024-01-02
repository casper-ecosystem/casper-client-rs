static GIT_DIR_NAME: &str = ".git";
static TARGET_DIR_NAME: &str = "target";

/// Build tar-gzip archive from files in the provided path.
pub fn build_archive(path: &Path) -> Result<Bytes, std::io::Error> {
    let buffer = BytesMut::new().writer();
    let encoder = GzEncoder::new(buffer, Compression::best());
    let mut archive = TarBuilder::new(encoder);

    for entry in (path.read_dir()?).flatten() {
        let file_name = entry.file_name();
        // Skip `.git` and `target`.
        if file_name == TARGET_DIR_NAME || file_name == GIT_DIR_NAME {
            continue;
        }
        let full_path = entry.path();
        if full_path.is_dir() {
            archive.append_dir_all(&file_name, &full_path)?;
        } else {
            archive.append_path_with_name(&full_path, &file_name)?;
        }
    }

    let encoder = archive.into_inner()?;
    let buffer = encoder.finish()?;
    Ok(buffer.into_inner().freeze())
}
