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
    analyzer.plot_magnitude("Lowpass 1kHz", "out/filters/lowpass_1k_mag.svg");
    analyzer.plot_phase("Lowpass 1kHz", "out/filters/lowpass_1k_phase.svg");
    analyzer.save_output("out/filters/lowpass_1k.wav");
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
    analyzer.plot_magnitude("Highpass 1kHz", "out/filters/highpass_1k_mag.svg");
    analyzer.plot_phase("Highpass 1kHz", "out/filters/highpass_1k_phase.svg");
    analyzer.save_output("out/filters/highpass_1k.wav");
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
    analyzer.plot_magnitude("Bandpass 1kHz", "out/filters/bandpass_1k_mag.svg");
    analyzer.plot_phase("Bandpass 1kHz", "out/filters/bandpass_1k_phase.svg");
    analyzer.save_output("out/filters/bandpass_1k.wav");
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
    analyzer.plot_magnitude("Peak 1kHz", "out/filters/peak_1k_mag.svg");
    analyzer.plot_phase("Peak 1kHz", "out/filters/peak_1k_phase.svg");
    analyzer.save_output("out/filters/peak_1k.wav");
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
    analyzer.plot_magnitude("Low shelf 1kHz", "out/filters/lowshelf_1k_mag.svg");
    analyzer.plot_phase("Low shelf 1kHz", "out/filters/lowshelf_1k_phase.svg");
    analyzer.save_output("out/filters/lowshelf_1k.wav");
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
    analyzer.plot_magnitude("High shelf 1kHz", "out/filters/highshelf_1k_mag.svg");
    analyzer.plot_phase("High shelf 1kHz", "out/filters/highshelf_1k_phase.svg");
    analyzer.save_output("out/filters/highshelf_1k.wav");
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
    analyzer.plot_magnitude("Notch 1kHz", "out/filters/notch_1k_mag.svg");
    analyzer.plot_phase("Notch 1kHz", "out/filters/notch_1k_phase.svg");
    analyzer.save_output("out/filters/notch_1k.wav");
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
    analyzer.plot_magnitude("Allpass 1kHz", "out/filters/allpass_1k_mag.svg");
    analyzer.plot_phase("Allpass 1kHz", "out/filters/allpass_1k_phase.svg");
    analyzer.save_output("out/filters/allpass_1k.wav");
}

#[test]
fn lowpass1p() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::Lowpass1p { freq: 1000.0 });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    analyzer.plot_magnitude("Lowpass one-pole 1kHz", "out/filters/lowpass1p_1k_mag.svg");
    analyzer.plot_phase(
        "Lowpass one-pole 1kHz",
        "out/filters/lowpass1p_1k_phase.svg",
    );
    analyzer.save_output("out/filters/lowpass1p_1k.wav");
}

#[test]
fn lowpass1p1z() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::Lowpass1p1z { freq: 1000.0 });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    analyzer.plot_magnitude(
        "Lowpass first order 1kHz",
        "out/filters/lowpass1p1z_1k_mag.svg",
    );
    analyzer.plot_phase(
        "Lowpass first order 1kHz",
        "out/filters/lowpass1p1z_1k_phase.svg",
    );
    analyzer.save_output("out/filters/lowpass1p1z_1k.wav");
}

#[test]
fn highpass1p1z() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::Highpass1p1z { freq: 1000.0 });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    analyzer.plot_magnitude(
        "Highpass first order 1kHz",
        "out/filters/highpass1p1z_1k_mag.svg",
    );
    analyzer.plot_phase(
        "Highpass first order 1kHz",
        "out/filters/highpass1p1z_1k_phase.svg",
    );
    analyzer.save_output("out/filters/highpass1p1z_1k.wav");
}

#[test]
fn lowshelf1st() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::LowShelf1st {
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
    analyzer.plot_magnitude(
        "Low shelf first order 1kHz",
        "out/filters/lowshelf1st_1k_mag.svg",
    );
    analyzer.plot_phase(
        "Low shelf first order 1kHz",
        "out/filters/lowshelf1st_1k_phase.svg",
    );
    analyzer.save_output("out/filters/lowshelf1st_1k.wav");
}

#[test]
fn highshelf1st() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::HighShelf1st {
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
    analyzer.plot_magnitude(
        "High shelf first order 1kHz",
        "out/filters/highshelf1st_1k_mag.svg",
    );
    analyzer.plot_phase(
        "High shelf first order 1kHz",
        "out/filters/highshelf1st_1k_phase.svg",
    );
    analyzer.save_output("out/filters/highshelf1st_1k.wav");
}

#[test]
fn allpass1st() {
    let mut filter = BiquadFilter2::new(SAMPLE_RATE);
    filter.set_params(FilterParams::Allpass1st { freq: 1000.0 });

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        filter.process_block(out_samples);
    });
    analyzer.plot_magnitude(
        "Allpass first order 1kHz",
        "out/filters/allpass1st_1k_mag.svg",
    );
    analyzer.plot_phase(
        "Allpass first order 1kHz",
        "out/filters/allpass1st_1k_phase.svg",
    );
    analyzer.save_output("out/filters/allpass1st_1k.wav");
}
