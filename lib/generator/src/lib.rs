const TAU: f64 = std::f64::consts::PI * 2.0;

/// Generator

pub trait Generator {
    fn next_sample(&mut self) -> f64;
}

/// OscillatorState

struct OscillatorState {
    frequency:   f64,
    sample_rate: f64,
    theta:       f64,
}

impl Default for OscillatorState {
    fn default() -> OscillatorState {
        OscillatorState {
            frequency:   440.0,
            sample_rate: 44100.0,
            theta:       0.0,
        }
    }
}

impl OscillatorState {
    fn get_frequency(&mut self) -> f64 {
        self.frequency
    }

    fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }

    fn get_sample_rate(&mut self) -> f64 {
        self.sample_rate
    }

    fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
    }

    fn advance(&mut self) {
        self.theta += TAU * self.frequency / self.sample_rate;
    }
}

/// Oscillator

pub struct Oscillator {
    state:      OscillatorState,
    gen_sample: fn(f64) -> f64,
}

impl Generator for Oscillator {
    fn next_sample(&mut self) -> f64 {
        let result = (self.gen_sample)(self.state.theta);
        self.state.advance();
        result
    }
}

impl Oscillator {
    pub fn sine(sample_rate: f64) -> Oscillator {
        Oscillator {
            state:      OscillatorState {
                            sample_rate: sample_rate,
                            .. OscillatorState::default()
                        },
            gen_sample: |theta: f64| -> f64 { theta.sin() },
        }
    }

    pub fn get_frequency(&mut self) -> f64 {
        self.state.get_frequency()
    }

    pub fn set_frequency(&mut self, frequency: f64) {
        self.state.set_frequency(frequency);
    }

    pub fn get_sample_rate(&mut self) -> f64 {
        self.state.get_sample_rate()
    }

    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        self.state.set_sample_rate(sample_rate);
    }
}
