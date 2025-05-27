use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::Path,
};

use anyhow::Context;
use walkdir::{DirEntry, WalkDir};
use zip::{result::ZipError, write::SimpleFileOptions};

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &Path,
    writer: T,
    method: zip::CompressionMethod,
) -> anyhow::Result<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = SimpleFileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let prefix = Path::new(prefix);
    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();
        let path_as_string = name
            .to_str()
            .map(str::to_owned)
            .with_context(|| format!("{name:?} Is a Non UTF-8 Path"))?;

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            zip.start_file(path_as_string, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            zip.add_directory(path_as_string, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

pub fn zip_directory(
    src_dir: &Path,
    dst_file: &Path,
    method: zip::CompressionMethod,
) -> anyhow::Result<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound.into());
    }

    let path = Path::new(dst_file);
    let file = File::create(path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn zip_works() {
        let src = Path::new("tests/resources/valid");
        let dest = Path::new("tests/resources/zip_test.zip");

        if dest.exists() {
            fs::remove_file(dest).unwrap()
        }

        let result = zip_directory(src, dest, zip::CompressionMethod::Deflated);

        assert_eq!(result.is_err(), false);

        result.unwrap();

        assert_eq!(dest.exists(), true);

        if dest.exists() {
            fs::remove_file(dest).unwrap()
        }
    }
}
