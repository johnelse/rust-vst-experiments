// lib.rs

#[macro_use] extern crate vst;
extern crate vstutils;

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{Category, CanDo, Info, Plugin};

use vstutils::generator::{Generator, Oscillator};
use vstutils::maths::{midi_pitch_to_freq};
use vstutils::notetracker::NoteTracker;
use vstutils::targetval::{Rate, TargetVal};

struct MonoSine {
    level:      TargetVal<f32>,
    velocity:   TargetVal<f32>,
    tracker:    NoteTracker,
    oscillator: Oscillator,
}

const ATTACK: f32 = 0.1;
const DECAY: f32 = 0.1;

impl MonoSine {
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1], data[2]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8, velocity: u8) {
        self.tracker.note_on(note);

        let target = velocity as f32 / 127.0;
        self.velocity.set_target(target);

        let time_per_sample = 1.0 / self.oscillator.get_sample_rate();
        self.velocity.set_inc_rate(Rate::Absolute(target
                                                  * time_per_sample / ATTACK));
        self.velocity.set_dec_rate(Rate::Absolute(target
                                                  * time_per_sample / DECAY));
    }

    fn note_off(&mut self, note: u8) {
        self.tracker.note_off(note);

        if self.get_current_note() == None {
            self.velocity.set_target(0.0);
        }
    }

    fn get_current_note(&self) -> Option<u8> {
        self.tracker
            .get_playing_notes()
            .get(0)
            .map(|note| note.to_owned())
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
            tracker:    NoteTracker::new(1, 9),
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
            0 => self.level.set_target(value),
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
        self.oscillator.set_sample_rate(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        if *self.velocity.get_value() > 0.0 || self.get_current_note() != None {
            if let Some(note) = self.get_current_note() {
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
