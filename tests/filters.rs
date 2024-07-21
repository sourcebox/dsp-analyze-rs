//! Tests for filters.

mod biquad;

use biquad::*;
use dsp_analyze::*;

/// Sample rate in Hz.
const SAMPLE_RATE: f32 = 48000.0;

/// Block size in samples.
const BLOCK_SIZE: usize = 16;

#[test]
fn lowpass() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::Lowpass {
        freq: 1000.0,
        q: 0.7,
    });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    // TODO: check why this plot ends up in an infinite loop.
    // analyzer.plot_magnitude("Lowpass 1kHz", "out/lowpass_1k_mag.svg");
    analyzer.plot_phase("Lowpass 1kHz", "out/lowpass_1k_phase.svg");
    analyzer.save_output("out/lowpass_1k.wav");
}

#[test]
fn highpass() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::Highpass {
        freq: 1000.0,
        q: 0.7,
    });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    analyzer.plot_magnitude("Highpass 1kHz", "out/highpass_1k_mag.svg");
    analyzer.plot_phase("Highpass 1kHz", "out/highpass_1k_phase.svg");
    analyzer.save_output("out/highpass_1k.wav");
}

#[test]
fn bandpass() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::Bandpass {
        freq: 1000.0,
        q: 0.7,
    });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    analyzer.plot_magnitude("Bandpass 1kHz", "out/bandpass_1k_mag.svg");
    analyzer.plot_phase("Bandpass 1kHz", "out/bandpass_1k_phase.svg");
    analyzer.save_output("out/bandpass_1k.wav");
}

#[test]
fn peak() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::Peak {
        freq: 1000.0,
        q: 0.7,
        gain: 20.0,
    });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    analyzer.plot_magnitude("Peak 1kHz", "out/peak_1k_mag.svg");
    analyzer.plot_phase("Peak 1kHz", "out/peak_1k_phase.svg");
    analyzer.save_output("out/peak_1k.wav");
}

#[test]
fn lowshelf() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::LowShelf {
        freq: 1000.0,
        gain: 20.0,
    });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    analyzer.plot_magnitude("Low shelf 1kHz", "out/lowshelf_1k_mag.svg");
    analyzer.plot_phase("Low shelf 1kHz", "out/lowshelf_1k_phase.svg");
    analyzer.save_output("out/lowshelf_1k.wav");
}

#[test]
fn highshelf() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::HighShelf {
        freq: 1000.0,
        gain: 20.0,
    });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    analyzer.plot_magnitude("High shelf 1kHz", "out/highshelf_1k_mag.svg");
    analyzer.plot_phase("High shelf 1kHz", "out/highshelf_1k_phase.svg");
    analyzer.save_output("out/highshelf_1k.wav");
}

#[test]
fn notch() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::Notch {
        freq: 1000.0,
        q: 0.7,
    });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    analyzer.plot_magnitude("Notch 1kHz", "out/notch_1k_mag.svg");
    analyzer.plot_phase("Notch 1kHz", "out/notch_1k_phase.svg");
    analyzer.save_output("out/notch_1k.wav");
}

#[test]
fn allpass() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::Allpass {
        freq: 1000.0,
        q: 0.7,
    });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    analyzer.plot_magnitude("Allpass 1kHz", "out/allpass_1k_mag.svg");
    analyzer.plot_phase("Allpass 1kHz", "out/allpass_1k_phase.svg");
    analyzer.save_output("out/allpass_1k.wav");
}
