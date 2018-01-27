// lib.rs

#[macro_use] extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Info, Plugin};

struct MonoSine {
    frequency: f32,
    level: f32,
    theta: f32,
    sample_rate: f32,
}

pub const TAU: f32 = std::f32::consts::PI * 2.0;

impl Default for MonoSine {
    fn default() -> MonoSine {
        MonoSine {
            frequency: 440.0,
            level: 1.0,
            theta: 0.0,
            sample_rate: 44100.0,
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

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.level,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.level = value,
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
        self.sample_rate = rate;
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples();
        let (_, outputs) = buffer.split();

        for output_buffer in outputs {
            let mut theta = self.theta;

            for output_sample in output_buffer {

                *output_sample = theta.sin() * self.level;
                theta += TAU * self.frequency / self.sample_rate;
            }
        }

        self.theta += samples as f32 *  TAU * self.frequency / self.sample_rate;
    }
}

plugin_main!(MonoSine);
