//! Writer for WAV files

use std::fs::File;
use std::path::Path;

/// Writes sample data as WAV file in 32-bit float format.
pub fn write(
    filename: impl AsRef<std::path::Path> + core::fmt::Display,
    sample_rate: u32,
    data: &[f32],
) -> std::io::Result<()> {
    let path = format!("{filename}");
    let path = Path::new(path.as_str());
    let parent = path.parent().unwrap();
    std::fs::create_dir_all(parent).ok();
    let mut file = File::create(path)?;
    let header = wav::Header::new(wav::WAV_FORMAT_IEEE_FLOAT, 1, sample_rate, 32);
    wav::write(header, &wav::BitDepth::from(Vec::from(data)), &mut file)?;
    Ok(())
}
