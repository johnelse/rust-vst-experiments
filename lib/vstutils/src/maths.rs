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
