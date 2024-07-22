//! Biquad IIR filters.

#![allow(unused)]

use core::f32::consts::PI;

use micromath::F32Ext;

/// Filter parameters.
#[derive(Debug, Default, Clone, PartialEq)]
pub enum FilterParams {
    /// Bypass mode.
    #[default]
    Bypass,

    /// Lowpass mode.
    Lowpass {
        /// Cutoff frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,
    },

    /// Highpass mode.
    Highpass {
        /// Cutoff frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,
    },

    /// Bandpass mode.
    Bandpass {
        /// Center frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,
    },

    /// Notch mode.
    Notch {
        /// Center frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,
    },

    /// Peak mode.
    Peak {
        /// Center frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,

        /// Gain in dB.
        gain: f32,
    },

    /// Low shelf mode.
    LowShelf {
        /// Base frequency in Hz.
        freq: f32,

        /// Gain in dB.
        gain: f32,
    },

    /// High shelf mode.
    HighShelf {
        /// Base frequency in Hz.
        freq: f32,

        /// Gain in dB.
        gain: f32,
    },

    /// Allpass mode.
    Allpass {
        /// Center frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,
    },

    /// One-pole lowpass mode.
    Lowpass1p {
        /// Cutoff frequency in Hz.
        freq: f32,
    },

    /// First order lowpass mode.
    Lowpass1p1z {
        /// Cutoff frequency in Hz.
        freq: f32,
    },

    /// First order highpass mode.
    Highpass1p1z {
        /// Cutoff frequency in Hz.
        freq: f32,
    },

    /// First order low shelf mode.
    LowShelf1st {
        /// Base frequency in Hz.
        freq: f32,

        /// Gain in dB.
        gain: f32,
    },

    /// First order high shelf mode.
    HighShelf1st {
        /// Base frequency in Hz.
        freq: f32,

        /// Gain in dB.
        gain: f32,
    },
}

impl FilterParams {
    /// Clamps all parameters to allowed ranges.
    pub fn clamp(&mut self, freq_range: (f32, f32), q_range: (f32, f32), gain_range: (f32, f32)) {
        match self {
            Self::Bypass => {}
            Self::Lowpass { freq, q }
            | Self::Highpass { freq, q }
            | Self::Bandpass { freq, q }
            | Self::Notch { freq, q }
            | Self::Allpass { freq, q } => {
                *freq = freq.clamp(freq_range.0, freq_range.1);
                *q = q.clamp(q_range.0, q_range.1);
            }
            Self::Peak { freq, q, gain } => {
                *freq = freq.clamp(freq_range.0, freq_range.1);
                *q = q.clamp(q_range.0, q_range.1);
                *gain = gain.clamp(gain_range.0, gain_range.1);
            }
            Self::LowShelf { freq, gain }
            | Self::HighShelf { freq, gain }
            | Self::LowShelf1st { freq, gain }
            | Self::HighShelf1st { freq, gain } => {
                *freq = freq.clamp(freq_range.0, freq_range.1);
                *gain = gain.clamp(gain_range.0, gain_range.1);
            }
            Self::Lowpass1p { freq } | Self::Lowpass1p1z { freq } | Self::Highpass1p1z { freq } => {
                *freq = freq.clamp(freq_range.0, freq_range.1);
            }
        }
    }
}

/// Normalized filter coefficients.
#[derive(Debug, Clone, PartialEq)]
pub struct BiquadFilterCoefficients {
    /// Coefficient a0 / b0.
    a0: f32,

    /// Coefficient a1 / b0.
    a1: f32,

    /// Coefficient a2 / b0.
    a2: f32,

    /// Coefficient b1 / b0.
    b1: f32,

    /// Coefficient b2 / b0.
    b2: f32,
}

impl Default for BiquadFilterCoefficients {
    fn default() -> Self {
        Self {
            a0: 1.0,
            a1: 0.0,
            a2: 0.0,
            b1: 0.0,
            b2: 0.0,
        }
    }
}

impl BiquadFilterCoefficients {
    /// Calculates the coefficients from the filter parameters.
    ///
    /// `sample_time` is `1.0 / sample_rate`.
    pub fn from_params(params: FilterParams, sample_time: f32) -> BiquadFilterCoefficients {
        match params {
            FilterParams::Bypass => BiquadFilterCoefficients::default(),
            FilterParams::Lowpass { freq, q } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (1.0 + k / q + k * k);
                let a0 = k * k * norm;
                Self {
                    a0,
                    a1: 2.0 * a0,
                    a2: a0,
                    b1: 2.0 * (k * k - 1.0) * norm,
                    b2: (1.0 - k / q + k * k) * norm,
                }
            }
            FilterParams::Highpass { freq, q } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (1.0 + k / q + k * k);
                let a0 = norm;
                Self {
                    a0,
                    a1: -2.0 * a0,
                    a2: a0,
                    b1: 2.0 * (k * k - 1.0) * norm,
                    b2: (1.0 - k / q + k * k) * norm,
                }
            }
            FilterParams::Bandpass { freq, q } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (1.0 + k / q + k * k);
                let a0 = k / q * norm;
                Self {
                    a0,
                    a1: 0.0,
                    a2: -a0,
                    b1: 2.0 * (k * k - 1.0) * norm,
                    b2: (1.0 - k / q + k * k) * norm,
                }
            }
            FilterParams::Notch { freq, q } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (1.0 + k / q + k * k);
                let a0 = (1.0 + k * k) * norm;
                let a1 = 2.0 * (k * k - 1.0) * norm;
                Self {
                    a0,
                    a1,
                    a2: a0,
                    b1: a1,
                    b2: (1.0 - k / q + k * k) * norm,
                }
            }
            FilterParams::Peak { freq, q, gain } => {
                let k = (PI * freq * sample_time).tan();
                let v = 10.0.powf(gain.abs() / 20.0);
                if gain >= 0.0 {
                    let norm = 1.0 / (1.0 + 1.0 / q * k + k * k);
                    let a1 = 2.0 * (k * k - 1.0) * norm;
                    Self {
                        a0: (1.0 + v / q * k + k * k) * norm,
                        a1,
                        a2: (1.0 - v / q * k + k * k) * norm,
                        b1: a1,
                        b2: (1.0 - 1.0 / q * k + k * k) * norm,
                    }
                } else {
                    let norm = 1.0 / (1.0 + v / q * k + k * k);
                    let a1 = 2.0 * (k * k - 1.0) * norm;
                    Self {
                        a0: (1.0 + 1.0 / q * k + k * k) * norm,
                        a1,
                        a2: (1.0 - 1.0 / q * k + k * k) * norm,
                        b1: a1,
                        b2: (1.0 - v / q * k + k * k) * norm,
                    }
                }
            }
            FilterParams::LowShelf { freq, gain } => {
                let k = (PI * freq * sample_time).tan();
                let v = 10.0.powf(gain.abs() / 20.0);
                if gain >= 0.0 {
                    let norm = 1.0 / (1.0 + 2.0.sqrt() * k + k * k);
                    Self {
                        a0: (1.0 + (2.0 * v).sqrt() * k + v * k * k) * norm,
                        a1: 2.0 * (v * k * k - 1.0) * norm,
                        a2: (1.0 - (2.0 * v).sqrt() * k + v * k * k) * norm,
                        b1: 2.0 * (k * k - 1.0) * norm,
                        b2: (1.0 - 2.0.sqrt() * k + k * k) * norm,
                    }
                } else {
                    let norm = 1.0 / (1.0 + (2.0 * v).sqrt() * k + v * k * k);
                    Self {
                        a0: (1.0 + 2.0.sqrt() * k + k * k) * norm,
                        a1: 2.0 * (k * k - 1.0) * norm,
                        a2: (1.0 - 2.0.sqrt() * k + k * k) * norm,
                        b1: 2.0 * (v * k * k - 1.0) * norm,
                        b2: (1.0 - (2.0 * v).sqrt() * k + v * k * k) * norm,
                    }
                }
            }
            FilterParams::HighShelf { freq, gain } => {
                let k = (PI * freq * sample_time).tan();
                let v = 10.0.powf(gain.abs() / 20.0);
                if gain >= 0.0 {
                    let norm = 1.0 / (1.0 + 2.0.sqrt() * k + k * k);
                    Self {
                        a0: (v + (2.0 * v).sqrt() * k + k * k) * norm,
                        a1: 2.0 * (k * k - v) * norm,
                        a2: (v - (2.0 * v).sqrt() * k + k * k) * norm,
                        b1: 2.0 * (k * k - 1.0) * norm,
                        b2: (1.0 - 2.0.sqrt() * k + k * k) * norm,
                    }
                } else {
                    let norm = 1.0 / (v + (2.0 * v).sqrt() * k + k * k);
                    Self {
                        a0: (1.0 + 2.0.sqrt() * k + k * k) * norm,
                        a1: 2.0 * (k * k - 1.0) * norm,
                        a2: (1.0 - 2.0.sqrt() * k + k * k) * norm,
                        b1: 2.0 * (k * k - v) * norm,
                        b2: (v - (2.0 * v).sqrt() * k + k * k) * norm,
                    }
                }
            }
            FilterParams::Allpass { freq, q } => {
                let k = (PI * freq * sample_time).tan();
                let div_q = 1.0 / q;
                let norm = 1.0 / (1.0 + k * div_q + k * k);
                let a0 = (1.0 - k * div_q + k * k) * norm;
                let a1 = 2.0 * (k * k - 1.0) * norm;
                Self {
                    a0,
                    a1,
                    a2: 1.0,
                    b1: a1,
                    b2: a0,
                }
            }
            FilterParams::Lowpass1p { freq } => {
                let b1 = (-2.0 * PI * freq * sample_time).exp();
                Self {
                    a0: 1.0 - b1,
                    a1: 0.0,
                    a2: 0.0,
                    b1: -b1,
                    b2: 0.0,
                }
            }
            FilterParams::Lowpass1p1z { freq } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (1.0 / k + 1.0);
                Self {
                    a0: norm,
                    a1: norm,
                    a2: 0.0,
                    b1: (1.0 - 1.0 / k) * norm,
                    b2: 0.0,
                }
            }
            FilterParams::Highpass1p1z { freq } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (k + 1.0);
                Self {
                    a0: norm,
                    a1: -norm,
                    a2: 0.0,
                    b1: (k - 1.0) * norm,
                    b2: 0.0,
                }
            }
            FilterParams::LowShelf1st { freq, gain } => {
                let k = (PI * freq * sample_time).tan();
                let v = 10.0.powf(gain.abs() / 20.0);
                if gain >= 0.0 {
                    let norm = 1.0 / (k + 1.0);
                    Self {
                        a0: (k * v + 1.0) * norm,
                        a1: (k * v - 1.0) * norm,
                        a2: 0.0,
                        b1: (k - 1.0) * norm,
                        b2: 0.0,
                    }
                } else {
                    let norm = 1.0 / (k * v + 1.0);
                    Self {
                        a0: (k + 1.0) * norm,
                        a1: (k - 1.0) * norm,
                        a2: 0.0,
                        b1: (k * v - 1.0) * norm,
                        b2: 0.0,
                    }
                }
            }
            FilterParams::HighShelf1st { freq, gain } => {
                let k = (PI * freq * sample_time).tan();
                let v = 10.0.powf(gain.abs() / 20.0);
                if gain >= 0.0 {
                    let norm = 1.0 / (k + 1.0);
                    Self {
                        a0: (k + v) * norm,
                        a1: (k - v) * norm,
                        a2: 0.0,
                        b1: (k - 1.0) * norm,
                        b2: 0.0,
                    }
                } else {
                    let norm = 1.0 / (k + v);
                    Self {
                        a0: (k + 1.0) * norm,
                        a1: (k - 1.0) * norm,
                        a2: 0.0,
                        b1: (k - v) * norm,
                        b2: 0.0,
                    }
                }
            }
        }
    }
}

/// Biquad IIR filter in direct form 1.
#[derive(Debug, Default, Clone)]
pub struct BiquadFilter1 {
    /// Time per sample, `1.0 / sample_rate`.
    sample_time: f32,

    /// Coefficients.
    coeffs: BiquadFilterCoefficients,

    /// Input sample memory.
    in_states: [f32; 2],

    /// Output sample memory.
    out_states: [f32; 2],
}

impl BiquadFilter1 {
    /// Returns a new instance.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_time: 1.0 / sample_rate,
            ..Default::default()
        }
    }

    /// Resets the filter to bypass mode.
    pub fn reset(&mut self) {
        self.set_params(FilterParams::Bypass);
    }

    /// Sets the coefficients according to the parameters.
    pub fn set_params(&mut self, params: FilterParams) {
        self.coeffs = BiquadFilterCoefficients::from_params(params, self.sample_time);
    }

    /// Sets the coefficients.
    pub fn set_coefficients(&mut self, coeffs: BiquadFilterCoefficients) {
        self.coeffs = coeffs;
    }

    /// Processes a single sample.
    pub fn process_sample(&mut self, sample: f32) -> f32 {
        let out_sample = self.coeffs.a0 * sample
            + self.coeffs.a1 * self.in_states[0]
            + self.coeffs.a2 * self.in_states[1]
            - self.coeffs.b1 * self.out_states[0]
            - self.coeffs.b2 * self.out_states[1];

        self.in_states[1] = self.in_states[0];
        self.in_states[0] = sample;

        self.out_states[1] = self.out_states[0];
        self.out_states[0] = out_sample;

        out_sample
    }

    /// Processes a block of samples in-place.
    pub fn process_block(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }
}

/// Biquad IIR filter in transposed direct form 2.
#[derive(Debug, Default, Clone)]
pub struct BiquadFilter2 {
    /// Time per sample, `1.0 / sample_rate`.
    sample_time: f32,

    /// Coefficients.
    coeffs: BiquadFilterCoefficients,

    /// Sample memory.
    states: [f32; 2],
}

impl BiquadFilter2 {
    /// Returns a new instance.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_time: 1.0 / sample_rate,
            ..Default::default()
        }
    }

    /// Resets the filter to bypass mode.
    pub fn reset(&mut self) {
        self.set_params(FilterParams::Bypass);
    }

    /// Sets the coefficients according to the parameters.
    pub fn set_params(&mut self, params: FilterParams) {
        self.coeffs = BiquadFilterCoefficients::from_params(params, self.sample_time);
    }

    /// Sets the coefficients.
    pub fn set_coefficients(&mut self, coeffs: BiquadFilterCoefficients) {
        self.coeffs = coeffs;
    }

    /// Processes a single sample.
    pub fn process_sample(&mut self, sample: f32) -> f32 {
        let out_sample = self.states[0] + self.coeffs.a0 * sample;

        self.states[0] = self.states[1] + self.coeffs.a1 * sample - self.coeffs.b1 * out_sample;
        self.states[1] = self.coeffs.a2 * sample - self.coeffs.b2 * out_sample;

        out_sample
    }

    /// Processes a block of samples in-place.
    pub fn process_block(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }
}
