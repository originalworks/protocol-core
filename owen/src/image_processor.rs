use log_macros::{format_error, log_info};
use rgb::FromSlice;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn optimize_image(path: &str) -> anyhow::Result<String> {
    log_info!("Optimizing image...");
    let path_obj = Path::new(path);

    let filename = path_obj
        .file_stem()
        .ok_or_else(|| format_error!("Failed to determine image filename"))?;

    let img = image::open(path)?
        .resize(720, 720, image::imageops::FilterType::CatmullRom)
        .into_rgba8();

    let img_pixels = img.as_raw().as_rgba();

    let img_data = ravif::Img::new(img_pixels, img.width() as usize, img.height() as usize);

    let avif_data = ravif::Encoder::new()
        .with_quality(50.0)
        .with_speed(7)
        .encode_rgba(img_data)?;

    let avif_path = path_obj
        .parent()
        .ok_or_else(|| format_error!("Failed to determine image path"))?
        .join(format!("{}.avif", filename.to_string_lossy()))
        .to_string_lossy()
        .to_string();

    let mut file = File::create(&avif_path)?;

    file.write_all(&avif_data.avif_file)?;

    Ok(avif_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_image() {
        let path = "tests/msg_one/e9a30355-79da-4dd4-acd9-797084bed79c/file.jpg";

        let new_path = optimize_image(path).unwrap();

        assert_eq!(
            &new_path,
            "tests/msg_one/e9a30355-79da-4dd4-acd9-797084bed79c/file.avif"
        );

        assert_eq!(std::path::Path::new(&new_path).exists(), true);
        std::fs::remove_file(new_path).unwrap();
    }
}
