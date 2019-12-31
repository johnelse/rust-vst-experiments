pub fn get_beats_frequencies(f_target: f32, f_beats: f32) -> (f32, f32) {
    // The centre frequency is related to the target frequency and
    // the beats frequency as follows:
    // (f_centre + f_beats / 2) * (f_centre - f_beats / 2) = f_target ^ 2
    let f_centre = (
                       f_beats * f_beats +
                       4. * f_target * f_target
                   ).sqrt() / 2.;

    (f_centre - f_beats / 2., f_centre + f_beats / 2.)
}

pub fn midi_pitch_to_freq(pitch: u8) -> f32 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f32 = 440.0;

    // Midi notes can be 0-127
    ((f32::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}

#[cfg(test)]
mod test {
    use super::*;

    fn floats_equal(first: f32, second: f32) -> bool {
        (first - second).abs() / ((first + second) / 2.) < 0.000001
    }

    #[test]
    fn test_get_beats_frequencies()
    {
        let f_target = 440.0;
        let f_beats  = 1.0;

        let (f_lower, f_upper) = get_beats_frequencies(f_target, f_beats);

        assert!(floats_equal(f_upper, f_lower + f_beats));
        assert!(floats_equal(f_lower * f_upper, f_target * f_target));
    }

    #[test]
    fn test_midi_pitch_to_freq() {
        assert!(floats_equal(midi_pitch_to_freq(57), 220.0));
        assert!(floats_equal(midi_pitch_to_freq(69), 440.0));
        assert!(floats_equal(midi_pitch_to_freq(81), 880.0));
    }
}
