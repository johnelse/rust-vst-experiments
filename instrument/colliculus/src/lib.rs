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
}

plugin_main!(Colliculus);
