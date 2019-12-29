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

    pub fn get_value(&self, position: f32) -> f32 {
        let table_size = self.values.len();
        let index0     = position as usize;
        let index1     = if index0 == (table_size - 1) {0} else {index0 + 1};
        let fraction   = position % 1.0;

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
    frequency:      f32,
    sample_rate:    f32,
    table_size_f:   f32,
    table_position: f32,
    table_rate:     f32,
}

impl OscillatorState {
    fn new(table_size: usize) -> OscillatorState {
        let mut state = OscillatorState {
            frequency:      440.0,
            sample_rate:    44100.0,
            table_size_f:   table_size as f32,
            table_position: 0.0,
            table_rate:     0.0,
        };
        state.update_table_rate();
        state
    }

    fn get_frequency(&mut self) -> f32 {
        self.frequency
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.update_table_rate();
    }

    fn get_sample_rate(&mut self) -> f32 {
        self.sample_rate
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.update_table_rate();
    }

    fn update_table_rate(&mut self) {
        self.table_rate = self.table_size_f * self.frequency / self.sample_rate;
    }

    fn next_position(&mut self) -> f32 {
        let position = self.table_position;

        self.table_position += self.table_rate;
        if self.table_position >= self.table_size_f {
            self.table_position -= self.table_size_f;
        }

        position
    }
}

/// Oscillator

pub struct Oscillator {
    state:      OscillatorState,
    wavetable:  WaveTable,
}

impl Generator for Oscillator {
    fn next_sample(&mut self) -> f32 {
        let position = self.state.next_position();
        self.wavetable.get_value(position)
    }
}

impl Oscillator {
    pub fn sine(sample_rate: f32) -> Oscillator {
        let table_size = 1024;
        let mut state = OscillatorState::new(table_size);
        state.set_sample_rate(sample_rate);

        Oscillator {
            state:      state,
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
