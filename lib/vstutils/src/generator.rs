const TAU: f32 = ::std::f32::consts::PI * 2.0;

/// Generator

pub trait Generator {
    fn next_sample(&mut self) -> f32;
}

/// OscillatorState

struct OscillatorState {
    frequency:   f32,
    sample_rate: f32,
    theta:       f32,
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
    fn get_frequency(&mut self) -> f32 {
        self.frequency
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    fn get_sample_rate(&mut self) -> f32 {
        self.sample_rate
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    fn advance(&mut self) {
        self.theta += TAU * self.frequency / self.sample_rate;
        self.theta %= TAU;
    }
}

/// Oscillator

pub struct Oscillator {
    state:      OscillatorState,
    gen_sample: fn(f32) -> f32,
}

impl Generator for Oscillator {
    fn next_sample(&mut self) -> f32 {
        let result = (self.gen_sample)(self.state.theta);
        self.state.advance();
        result
    }
}

impl Oscillator {
    pub fn sine(sample_rate: f32) -> Oscillator {
        Oscillator {
            state:      OscillatorState {
                            sample_rate: sample_rate,
                            .. OscillatorState::default()
                        },
            gen_sample: |theta: f32| -> f32 { theta.sin() },
        }
    }

    pub fn get_frequency(&mut self) -> f32 {
        self.state.get_frequency()
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.state.set_frequency(frequency);
    }

    pub fn get_sample_rate(&mut self) -> f32 {
        self.state.get_sample_rate()
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.state.set_sample_rate(sample_rate);
    }
}
