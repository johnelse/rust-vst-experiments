// lib.rs

#[macro_use] extern crate vst;
extern crate vstutils;

use vst::api::{Events, Supported, TimeInfoFlags};
use vst::buffer::AudioBuffer;
use vst::event::{Event, MidiEvent};
use vst::host::Host;
use vst::plugin::{Category, CanDo, HostCallback, Info, Plugin};

use vstutils::convert::{midi_pitch_to_freq};
use vstutils::division;
use vstutils::division::Division;
use vstutils::generator::{Generator, Oscillator};
use vstutils::targetval::{Rate, TargetVal};

struct Colliculus {
    host:           HostCallback,
    level:          TargetVal<f32>,
    pan:            TargetVal<f32>,
    velocity:       TargetVal<f32>,
    division_param: f32,
    division:       Division,
    note:           Option<u8>,
    osc1:           Oscillator,
    osc2:           Oscillator,
}

const ATTACK: f32 = 0.1;
const DECAY: f32 = 0.1;

impl Colliculus {
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1], data[2]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8, velocity: u8) {
        self.note = Some(note);

        let target = velocity as f32 / 127.0;
        self.velocity.set_target(target);

        let time_per_sample = 1.0 / self.osc1.get_sample_rate();
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

impl Default for Colliculus {
    fn default() -> Colliculus {
        Colliculus::new(Default::default())
    }
}

impl Plugin for Colliculus {
    fn new(host: HostCallback) -> Colliculus {
        Colliculus {
            host:           host,
            level:          TargetVal::new(  Rate::Relative(0.001)
                                           , Rate::Relative(0.001)
                                           , 1.0),
            pan:            TargetVal::new(  Rate::Relative(0.001)
                                           , Rate::Relative(0.001)
                                           , 0.5),
            velocity:       TargetVal::new(  Rate::Absolute(0.0)
                                           , Rate::Absolute(0.0)
                                           , 0.0),
            division_param: 0.0,
            division:       division::get_division(0.0),
            note:           None,
            osc1:           Oscillator::sine(44100.0),
            osc2:           Oscillator::sine(44100.0),
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name:       "Colliculus".to_string(),
            vendor:     "johnelse".to_string(),
            unique_id:  13052018,

            inputs:     0,
            outputs:    2,
            parameters: 3,

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
            1 => *self.pan.get_target() as f32,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.level.set_target(value),
            1 => self.pan.set_target(value),
            2 => {
                self.division_param = value;
                self.division       = division::get_division(self.division_param)
            },
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Level".to_string(),
            1 => "Pan".to_string(),
            2 => "Division".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // Convert to a percentage
            0 => format!("{}", self.level.get_target() * 100.0),
            1 => format!("{}", (self.pan.get_target() - 0.5) * 100.0),
            2 => division::get_name(self.division),
            _ => "".to_string(),
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.osc1.set_sample_rate(rate);
        self.osc2.set_sample_rate(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        if *self.velocity.get_value() > 0.0 || self.note != None {
            if let Some(note) = self.note {
                let f_beats = match self.host.get_time_info(TimeInfoFlags::TEMPO_VALID.bits()) {
                    None            => 120.,
                    Some(time_info) => {
                         time_info.tempo as f32
                    },
                } * division::get_tempo_multiplier(self.division);

                let f_target = midi_pitch_to_freq(note);

                // The centre frequency is related to the target frequency and
                // the beats frequency as follows:
                // (f_centre + f_beats / 2) * (f_centre - f_beats / 2) = f_target ^ 2
                let f_centre = (
                                   f_beats * f_beats +
                                   4. * f_target * f_target
                               ).sqrt() / 2.;

                self.osc1.set_frequency(f_centre + f_beats / 2.);
                self.osc2.set_frequency(f_centre - f_beats / 2.);
            }

            let samples = buffer.samples();
            let (_, outputs) = buffer.split();

            if outputs.len() == 2 {
                for sample_index in 0..samples {
                    self.level.advance();
                    self.pan.advance();
                    self.velocity.advance();

                    let osc1_value = self.osc1.next_sample();
                    let osc2_value = self.osc2.next_sample();

                    let pan        = self.pan.get_value();
                    let osc1_left  = 1.0 - pan;
                    let osc2_left  = pan;
                    let osc1_right = pan;
                    let osc2_right = 1.0 - pan;

                    if let Some (left_sample) = outputs.get_mut(0).get_mut(sample_index) {
                        *left_sample =   (osc1_value * osc1_left + osc2_value * osc2_left)
                                       * self.level.get_value()
                                       * self.velocity.get_value();
                    }
                    if let Some (right_sample) = outputs.get_mut(1).get_mut(sample_index) {
                        *right_sample =   (osc1_value * osc1_right + osc2_value * osc2_right)
                                        * self.level.get_value()
                                        * self.velocity.get_value();
                    }
                }
            }
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(MidiEvent {data, ..}) => self.process_midi_event(data),
                _ => (),
            }
        }
    }
}

plugin_main!(Colliculus);
