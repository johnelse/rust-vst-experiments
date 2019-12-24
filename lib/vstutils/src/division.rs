#[derive(Clone, Copy)]
pub enum Division {
    WholeDot,
    Whole,
    WholeTriplet,
    HalfDot,
    Half,
    HalfTriplet,
    QuarterDot,
    Quarter,
    QuarterTriplet,
    EighthDot,
    Eighth,
    EighthTriplet,
    SixteenthDot,
    Sixteenth,
    SixteenthTriplet,
}

const NUM_DIVISIONS: u8 = 15;

pub fn get_division(param: f32) -> Division {
    let clamped_param =
        if      param < 0.0 {0.0}
        else if param > 1.0 {1.0}
        else                {param};

    match (clamped_param * NUM_DIVISIONS as f32) as u8 {
        0  => Division::WholeDot,
        1  => Division::Whole,
        2  => Division::WholeTriplet,
        3  => Division::HalfDot,
        4  => Division::Half,
        5  => Division::HalfTriplet,
        6  => Division::QuarterDot,
        7  => Division::Quarter,
        8  => Division::QuarterTriplet,
        9  => Division::EighthDot,
        10 => Division::Eighth,
        11 => Division::EighthTriplet,
        12 => Division::SixteenthDot,
        13 => Division::Sixteenth,
        _  => Division::SixteenthTriplet,
    }
}

pub fn get_name(division: Division) -> String {
    match division {
        Division::WholeDot         => "1."   .to_string(),
        Division::Whole            => "1"    .to_string(),
        Division::WholeTriplet     => "1T"   .to_string(),
        Division::HalfDot          => "1/2." .to_string(),
        Division::Half             => "1/2"  .to_string(),
        Division::HalfTriplet      => "1/2T" .to_string(),
        Division::QuarterDot       => "1/4." .to_string(),
        Division::Quarter          => "1/4"  .to_string(),
        Division::QuarterTriplet   => "1/4T" .to_string(),
        Division::EighthDot        => "1/8." .to_string(),
        Division::Eighth           => "1/8"  .to_string(),
        Division::EighthTriplet    => "1/8T" .to_string(),
        Division::SixteenthDot     => "1/16.".to_string(),
        Division::Sixteenth        => "1/16" .to_string(),
        Division::SixteenthTriplet => "1/16T".to_string(),
    }
}

pub fn get_tempo_multiplier(division: Division) -> f32 {
    let beat_multiplier : f32 = match division {
        Division::WholeDot         => 0.5 / 3.0,
        Division::Whole            => 0.25,
        Division::WholeTriplet     => 0.375,
        Division::HalfDot          => 1.0 / 3.0,
        Division::Half             => 0.5,
        Division::HalfTriplet      => 0.75,
        Division::QuarterDot       => 2.0 / 3.0,
        Division::Quarter          => 1.0,
        Division::QuarterTriplet   => 1.5,
        Division::EighthDot        => 4.0 / 3.0,
        Division::Eighth           => 2.0,
        Division::EighthTriplet    => 3.0,
        Division::SixteenthDot     => 8.0 / 3.0,
        Division::Sixteenth        => 4.0,
        Division::SixteenthTriplet => 6.0,
    };

    beat_multiplier / 60.0
}
