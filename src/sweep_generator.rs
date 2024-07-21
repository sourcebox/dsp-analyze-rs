//! Sine sweep generator.

#![allow(unused)]

/// Sweep generator.
#[derive(Debug, Default)]
pub struct SweepGenerator {
    /// Sample rate in Hz.
    sample_rate: f32,

    /// Minimum frequency.
    min_freq: f32,

    /// Maximum frequency.
    max_freq: f32,

    /// Sweep time in seconds.
    sweep_time: f32,

    /// Gain.
    gain: f32,

    /// Current phase.
    phase: f32,

    /// Phase increment.
    phase_inc: f32,

    /// Current frequency.
    freq: f32,

    /// Frequency increment.
    freq_inc: f32,

    /// Started flag.
    started: bool,
}

/// Error variants.
#[derive(Debug)]
pub enum SweepError {
    /// Sweep was not started.
    NotStarted,

    /// Maximum frequency reached.
    MaxFreqReached,
}

/// Custom result type.
pub type Result<T> = core::result::Result<T, SweepError>;

impl SweepGenerator {
    /// Returns a new instance.
    pub fn new(sample_rate: f32) -> Self {
        let min_freq = 20.0;
        let max_freq = 20000.0;
        let sweep_time = 1.0;
        let freq_inc = (max_freq - min_freq) / (sample_rate * sweep_time);

        Self {
            sample_rate,
            min_freq,
            max_freq,
            sweep_time,
            gain: 1.0,
            freq: min_freq,
            freq_inc,
            ..Default::default()
        }
    }

    /// Sets the sweep range in Hz.
    pub fn set_range(&mut self, min_freq: f32, max_freq: f32) {
        self.min_freq = min_freq;
        self.max_freq = max_freq;
        self.freq_inc = (self.max_freq - self.min_freq) / (self.sample_rate * self.sweep_time);
    }

    /// Sets the sweep time in seconds.
    pub fn set_time(&mut self, sweep_time: f32) {
        self.sweep_time = sweep_time;
        self.freq_inc = (self.max_freq - self.min_freq) / (self.sample_rate * self.sweep_time);
    }

    /// Sets the gain.
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    /// Starts the sweep.
    pub fn start(&mut self) {
        self.freq = self.min_freq;
        self.phase_inc = core::f32::consts::TAU * self.freq / self.sample_rate;
        self.started = true;
    }

    /// Generates a block of samples.
    pub fn process(&mut self, buffer: &mut [f32]) -> Result<()> {
        if !self.started {
            return Err(SweepError::NotStarted);
        }

        for sample in buffer.iter_mut() {
            if self.freq > self.max_freq {
                self.started = false;
                return Err(SweepError::MaxFreqReached);
            }

            *sample = self.phase.sin() * self.gain;

            self.phase += self.phase_inc;

            // Wrap phase to avoid float precision issues.
            if self.phase > core::f32::consts::TAU {
                self.phase -= core::f32::consts::TAU;
            }

            // Calculate new frequency and phase increment.
            self.freq += self.freq_inc;
            self.phase_inc = core::f32::consts::TAU * self.freq / self.sample_rate;
        }

        Ok(())
    }
}
