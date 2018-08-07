// lib.rs

#[macro_use] extern crate vst;
extern crate vstutils;

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{Category, CanDo, Info, Plugin};

use vstutils::convert::{midi_pitch_to_freq};
use vstutils::generator::{Generator, Oscillator};
use vstutils::targetval::{Rate, TargetVal};

struct Colliculus {
    level:    TargetVal<f64>,
    pan:      TargetVal<f64>,
    velocity: TargetVal<f64>,
    note:     Option<u8>,
    osc1:     Oscillator,
    osc2:     Oscillator,
}

const ATTACK: f64 = 0.1;
const DECAY: f64 = 0.1;

impl Default for Colliculus {
    fn default() -> Colliculus {
        Colliculus {
            level:    TargetVal::new(  Rate::Relative(0.001)
                                     , Rate::Relative(0.001)
                                     , 1.0),
            pan:      TargetVal::new(  Rate::Relative(0.001)
                                     , Rate::Relative(0.001)
                                     , 0.0),
            velocity: TargetVal::new(  Rate::Absolute(0.0)
                                     , Rate::Absolute(0.0)
                                     , 0.0),
            note:     None,
            osc1:     Oscillator::sine(44100.0),
            osc2:     Oscillator::sine(44100.0),
        }
    }
}

impl Plugin for Colliculus {
    fn get_info(&self) -> Info {
        Info {
            name:       "Colliculus".to_string(),
            vendor:     "johnelse".to_string(),
            unique_id:  13052018,

            inputs:     0,
            outputs:    2,
            parameters: 4,

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
            1 => (*self.pan.get_target() as f32 + 1.0) / 2.0,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.level.set_target(value as f64),
            1 => self.pan.set_target(value as f64 * 2.0 - 1.0),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Level".to_string(),
            1 => "Pan".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // Convert to a percentage
            0 => format!("{}", self.level.get_target() * 100.0),
            1 => format!("{}", self.pan.get_target() * 50.0),
            _ => "".to_string(),
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.osc1.set_sample_rate(rate as f64);
        self.osc2.set_sample_rate(rate as f64);
    }
}

plugin_main!(Colliculus);
