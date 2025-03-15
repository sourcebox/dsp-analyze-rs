//! Writer for WAV files

use std::path::Path;

use hound::*;

/// Writes sample data as WAV file in 32-bit float format.
pub fn write(
    filename: impl AsRef<std::path::Path> + core::fmt::Display,
    sample_rate: u32,
    samples: &[f32],
) -> std::io::Result<()> {
    let path = format!("{filename}");
    let path = Path::new(path.as_str());

    // Create parent directories to the path if they don't exist.
    let parent = path.parent().unwrap();
    std::fs::create_dir_all(parent).ok();

    let spec = WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };
    let mut writer = WavWriter::create(path, spec).unwrap();

    for sample in samples {
        writer.write_sample(*sample).unwrap();
        writer.write_sample(*sample).unwrap();
    }

    Ok(())
}
