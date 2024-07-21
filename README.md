# dsp-analyze

This crate performs basic audio dsp analysis.

**WARNING**: It is currently in a very early state and needs contributions to be improved.

Currently implemented:

- Frequency response and phase plots using FFT (bode diagrams), 48kHz sample rate only.

## Usage

```rust no_run
use dsp_analyze::*;

let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
    sample_rate: 48000.0,
    block_size: 256,
    ..Default::default()
});

analyzer.run(|in_samples, out_samples| {
    // Do something useful here.
});

analyzer.plot_magnitude("Test Plot", "out/test_mag.svg");
analyzer.plot_phase("Test Plot", "out/test_phase.svg");
```

## License

Published under the MIT license. Any contribution to this project must be provided under the same license conditions.
