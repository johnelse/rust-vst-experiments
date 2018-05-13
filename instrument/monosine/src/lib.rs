// lib.rs

extern crate generator;
extern crate targetval;
#[macro_use] extern crate vst;

use generator::{Generator, Oscillator};
use targetval::{Rate, TargetVal};

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{Category, CanDo, Info, Plugin};

struct MonoSine {
    level:      TargetVal<f64>,
    velocity:   TargetVal<f64>,
    note:       Option<u8>,
    oscillator: Oscillator,
}

const ATTACK: f64 = 0.1;
const DECAY: f64 = 0.1;

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
            144 => self.note_on(data[1], data[2]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8, velocity: u8) {
        self.note = Some(note);

        let target = velocity as f64 / 127.0;
        self.velocity.set_target(target);

        let time_per_sample = 1.0 / self.oscillator.get_sample_rate();
        self.velocity.set_inc_rate(Rate::Absolute(target
                                                  * time_per_sample / ATTACK));
        self.velocity.set_dec_rate(Rate::Absolute(target
                                                  * time_per_sample / DECAY));
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None;
            self.velocity.set_target(0.0);
        }
    }
}

impl Default for MonoSine {
    fn default() -> MonoSine {
        MonoSine {
            level:      TargetVal::new(  Rate::Relative(0.001)
                                       , Rate::Relative(0.001)
                                       , 1.0),
            velocity:   TargetVal::new(  Rate::Absolute(0.0)
                                       , Rate::Absolute(0.0)
                                       , 0.0),
            note:       None,
            oscillator: Oscillator::sine(44100.0),
        }
    }
}

impl Plugin for MonoSine {
    fn get_info(&self) -> Info {
        Info {
            name:       "MonoSine".to_string(),
            vendor:     "johnelse".to_string(),
            unique_id:  20012018,

            inputs:     0,
            outputs:    2,
            parameters: 1,

            category:   Category::Synth,

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
            0 => *self.level.get_target() as f32,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.level.set_target(value as f64),
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
            0 => format!("{}", self.level.get_target() * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, _: i32) -> String {
        "".to_string()
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.oscillator.set_sample_rate(rate as f64);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        if *self.velocity.get_value() > 0.0 || self.note != None {
            if let Some(note) = self.note {
                self.oscillator.set_frequency(midi_pitch_to_freq(note));
            }

            let samples = buffer.samples();
            let (_, outputs) = buffer.split();

            for sample_index in 0..samples {
                self.level.advance();
                self.velocity.advance();

                for output_buffer in outputs {
                    if let Some(output_sample) = output_buffer.get_mut(sample_index) {
                        *output_sample = (  self.oscillator.next_sample()
                                          * self.level.get_value()
                                          * self.velocity.get_value()) as f32;
                    }
                }
            }

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
