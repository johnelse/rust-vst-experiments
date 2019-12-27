const TAU: f32 = ::std::f32::consts::PI * 2.0;

/// WaveTable

pub struct WaveTable {
    values: Box<[f32]>,
}

impl WaveTable {
    pub fn new(size: usize, gen_sample: fn(f32) -> f32) -> WaveTable {
        let mut values = Vec::with_capacity(size);
        let size_f = size as f32;

        for i in 0..size {
            values.push(gen_sample(TAU * (i as f32) / (size_f)));
        }

        WaveTable {
            values: values.into_boxed_slice(),
        }
    }

    /// This method assumes 0 <= theta and theta < tau
    pub fn get_value(&self, theta: f32) -> f32
    {
        let table_size = self.values.len();
        let position   = theta / TAU * (table_size as f32);
        let index0     = position as usize;
        let index1     = if index0 == (table_size - 1) {0} else {index0 + 1};
        let fraction   = position - (index0 as f32);

        let value0 = self.values[index0];
        let value1 = self.values[index1];

        value0 + (value1 - value0) * fraction
    }
}

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
    wavetable:  WaveTable,
}

impl Generator for Oscillator {
    fn next_sample(&mut self) -> f32 {
        let result = self.wavetable.get_value(self.state.theta);
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
            wavetable:  WaveTable::new(1024, |theta: f32| -> f32 { theta.sin() }),
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
