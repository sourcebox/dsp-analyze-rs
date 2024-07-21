//! FFT analyzer with Bode plots showing mangnitude and phase.

use plotters::prelude::*;
use realfft::{num_complex::Complex, RealFftPlanner};

use crate::plot::{AxisRange, Plot, Series};
use crate::sweep_generator::SweepGenerator;
use crate::wav_writer;

/// Configuration for the analyzer.
#[derive(Debug, Clone)]
pub struct FftAnalyzerConfig {
    /// Sample rate in Hz.
    pub sample_rate: f32,

    /// Block size for each process call.
    pub block_size: usize,
}

impl Default for FftAnalyzerConfig {
    /// Returns the default configuration for the plotter:
    /// - Sample rate: 48kHz
    /// - Block size: 64 samples
    fn default() -> Self {
        Self {
            sample_rate: 48000.0,
            block_size: 64,
        }
    }
}

/// FFT analyzer.
#[derive(Debug)]
pub struct FftAnalyzer {
    /// Current configuration.
    pub config: FftAnalyzerConfig,

    /// Input samples.
    pub in_samples: Vec<f32>,

    /// Output samples.
    pub out_samples: Vec<f32>,

    /// Magnitude of the spectrum.
    pub spectrum_magnitude: Vec<f32>,

    /// Phase of the spectrum.
    pub spectrum_phase: Vec<f32>,
}

impl FftAnalyzer {
    /// Returns a new instance of the analyzer.
    pub fn new(config: FftAnalyzerConfig) -> Self {
        Self {
            config,
            in_samples: Vec::new(),
            out_samples: Vec::new(),
            spectrum_magnitude: Vec::new(),
            spectrum_phase: Vec::new(),
        }
    }

    /// Clears the spectrum data.
    pub fn clear(&mut self) {
        self.in_samples.clear();
        self.out_samples.clear();
        self.spectrum_magnitude.clear();
        self.spectrum_phase.clear();
    }

    /// Runs the test signal through the provided function and
    /// analyzes the result.
    ///
    /// The function is called in a loop, each iteration is passed
    /// the number of samples equal to the configured block size.
    ///
    /// The function closure takes two arguments:
    /// - The first is a read-only buffer containing the samples of the test signal.
    /// - The second argument is a writable buffer for the processed samples.
    ///   It is initially filled with a copy of the input samples.
    pub fn run<F>(&mut self, mut func: F)
    where
        F: FnMut(&[f32], &mut [f32]),
    {
        self.clear();

        self.in_samples = sweep(self.config.sample_rate);
        self.out_samples.clone_from(&self.in_samples);
        let chunk_size = self.config.block_size;

        for (in_samples, out_samples) in self
            .in_samples
            .chunks(chunk_size)
            .zip(self.out_samples.chunks_mut(chunk_size))
        {
            func(in_samples, out_samples);
        }

        let in_spectrum = fft(&self.in_samples);
        let out_spectrum = fft(&self.out_samples);
        let spectrum = out_spectrum.iter().zip(in_spectrum).map(|v| v.0 / v.1);

        self.spectrum_magnitude = spectrum
            .clone()
            .map(|v| 20.0 * f32::log10(v.norm()))
            .collect();
        self.spectrum_phase = spectrum
            .map(|v| v.arg() / std::f32::consts::PI * 180.0)
            .collect();
    }

    /// Saves the input signal as WAV file.
    pub fn save_input(&self, filename: impl AsRef<std::path::Path> + core::fmt::Display) {
        wav_writer::write(filename, self.config.sample_rate as u32, &self.in_samples).unwrap();
    }

    /// Saves the output signal as WAV file.
    pub fn save_output(&self, filename: impl AsRef<std::path::Path> + core::fmt::Display) {
        wav_writer::write(filename, self.config.sample_rate as u32, &self.out_samples).unwrap();
    }

    /// Plots the magnitude as SVG file.
    pub fn plot_magnitude(&self, title: &str, filename: impl AsRef<std::path::Path>) {
        Plot {
            title,
            bode: true,
            series: &[Series {
                label: "Magnitude",
                samplerate: self.config.sample_rate,
                series: self.spectrum_magnitude.as_slice(),
                color: &BLUE,
            }],
            y_range: AxisRange::AutoLin,
        }
        .create_svg(filename);
    }

    /// Plots the phase as SVG file.
    pub fn plot_phase(&self, title: &str, filename: impl AsRef<std::path::Path>) {
        Plot {
            title,
            bode: true,
            series: &[Series {
                label: "Phase",
                samplerate: self.config.sample_rate,
                series: self.spectrum_phase.as_slice(),
                color: &RED,
            }],
            y_range: AxisRange::ManualLin(-180.0..180.0),
        }
        .create_svg(filename);
    }
}

/// Returns a `Vec` of sweep samples.
fn sweep(sample_rate: f32) -> Vec<f32> {
    let mut sweep_generator = SweepGenerator::new(sample_rate);
    sweep_generator.set_range(1.0, 20000.0);
    sweep_generator.set_time(1.0);
    sweep_generator.start();

    let mut buffer = [0.0; 16];
    let mut samples = Vec::new();

    while sweep_generator.process(&mut buffer).is_ok() {
        samples.extend(buffer);
    }

    samples
}

/// Runs the FFT over the input samples and returns the spectrum.
fn fft(indata: &[f32]) -> Vec<Complex<f32>> {
    // Make a planner.
    let mut real_planner = RealFftPlanner::<f32>::new();

    // Create an FFT.
    let r2c = real_planner.plan_fft_forward(indata.len());

    // Make a vector for storing the spectrum.
    let mut spectrum = r2c.make_output_vec();

    // Forward transform the signal.
    let mut indata = indata.to_owned();
    r2c.process(&mut indata, &mut spectrum).unwrap();
    let scale = 1.0 / (spectrum.len() as f32).sqrt();

    for v in spectrum.iter_mut() {
        *v *= scale;
    }

    spectrum
}
