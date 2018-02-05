// lib.rs

#[macro_use] extern crate vst;

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{Category, CanDo, Info, Plugin};

struct MonoSine {
    level: f64,
    target_velocity: f64,
    velocity: f64,
    theta: f64,
    sample_rate: f64,
    note: Option<u8>,
    frequency: f64,
}

pub const TAU: f64 = std::f64::consts::PI * 2.0;
pub const ATTACK: f64 = 0.1;
pub const DECAY: f64 = 0.1;

fn midi_pitch_to_freq(pitch: u8) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    // Midi notes can be 0-127
    ((f64::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}

impl MonoSine {
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8) {
        self.note = Some(note);
        self.target_velocity = 1.0;
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None;
        }
    }
}

impl Default for MonoSine {
    fn default() -> MonoSine {
        MonoSine {
            level: 1.0,
            velocity: 0.0,
            target_velocity: 0.0,
            theta: 0.0,
            sample_rate: 44100.0,
            note: None,
            frequency: 440.0,
        }
    }
}

impl Plugin for MonoSine {
    fn get_info(&self) -> Info {
        Info {
            name: "MonoSine".to_string(),
            vendor: "johnelse".to_string(),
            unique_id: 20012018,

            inputs: 0,
            outputs: 2,
            parameters: 1,

            category: Category::Synth,

            // fill in the rest with the default values
            ..Info::default()
        }
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.level as f32,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.level = value as f64,
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Level".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // Convert to a percentage
            0 => format!("{}", self.level * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, _: i32) -> String {
        "".to_string()
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate as f64;
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let time_per_sample = 1.0 / self.sample_rate;

        if self.velocity > 0.0 || self.note != None {
            if let Some(note) = self.note {
                self.frequency = midi_pitch_to_freq(note);
            }

            let samples = buffer.samples();
            let (_, outputs) = buffer.split();

            for output_buffer in outputs {
                let mut theta = self.theta;
                let mut velocity = self.velocity;

                for output_sample in output_buffer {
                    if None == self.note {
                        if velocity > 0.0 {
                            velocity -=
                                self.target_velocity * time_per_sample / DECAY;
                        }
                    }
                    else {
                        if velocity < self.target_velocity {
                            velocity +=
                                self.target_velocity * time_per_sample / ATTACK;
                        }
                    }

                    *output_sample = (theta.sin() * self.level * velocity) as f32;
                    theta += TAU * self.frequency / self.sample_rate;
                }
            }

            if None == self.note {
                self.velocity -= self.target_velocity * samples as f64 * time_per_sample / DECAY;
                if self.velocity < 0.0 {
                    self.velocity = 0.0;
                }
            } else {
                self.velocity += self.target_velocity * samples as f64 * time_per_sample / ATTACK;
                if self.velocity > self.target_velocity {
                    self.velocity = self.target_velocity;
                }
            }

            self.theta += samples as f64 * TAU * self.frequency / self.sample_rate;
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                _ => (),
            }
        }
    }
}

plugin_main!(MonoSine);
