use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Linear,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    QuintIn,
    QuintOut,
    QuintInOut,
    SineIn,
    SineOut,
    SineInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    CircIn,
    CircOut,
    CircInOut,
    BackIn,
    BackOut,
    BackInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
    Custom { bezier: [f32; 4] },
}

impl Easing {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Self::Linear => t,
            Self::QuadIn => quad_in(t),
            Self::QuadOut => quad_out(t),
            Self::QuadInOut => quad_in_out(t),
            Self::CubicIn => cubic_in(t),
            Self::CubicOut => cubic_out(t),
            Self::CubicInOut => cubic_in_out(t),
            Self::QuartIn => quart_in(t),
            Self::QuartOut => quart_out(t),
            Self::QuartInOut => quart_in_out(t),
            Self::QuintIn => quint_in(t),
            Self::QuintOut => quint_out(t),
            Self::QuintInOut => quint_in_out(t),
            Self::SineIn => sine_in(t),
            Self::SineOut => sine_out(t),
            Self::SineInOut => sine_in_out(t),
            Self::ExpoIn => expo_in(t),
            Self::ExpoOut => expo_out(t),
            Self::ExpoInOut => expo_in_out(t),
            Self::CircIn => circ_in(t),
            Self::CircOut => circ_out(t),
            Self::CircInOut => circ_in_out(t),
            Self::BackIn => back_in(t),
            Self::BackOut => back_out(t),
            Self::BackInOut => back_in_out(t),
            Self::ElasticIn => elastic_in(t),
            Self::ElasticOut => elastic_out(t),
            Self::ElasticInOut => elastic_in_out(t),
            Self::BounceIn => bounce_in(t),
            Self::BounceOut => bounce_out(t),
            Self::BounceInOut => bounce_in_out(t),
            Self::Custom { bezier } => cubic_bezier(t, bezier[0], bezier[1], bezier[2], bezier[3]),
        }
    }
}

impl Default for Easing {
    fn default() -> Self {
        Self::CubicOut
    }
}

#[must_use]
pub fn quad_in(t: f32) -> f32 {
    t * t
}

#[must_use]
pub fn quad_out(t: f32) -> f32 {
    -t * (t - 2.0)
}

#[must_use]
pub fn quad_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -2.0 * t * t + 4.0 * t - 1.0
    }
}

#[must_use]
pub fn cubic_in(t: f32) -> f32 {
    t * t * t
}

#[must_use]
pub fn cubic_out(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
}

#[must_use]
pub fn cubic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t = 2.0 * t - 2.0;
        0.5 * t * t * t + 1.0
    }
}

#[must_use]
pub fn quart_in(t: f32) -> f32 {
    t * t * t * t
}

#[must_use]
pub fn quart_out(t: f32) -> f32 {
    let t = t - 1.0;
    -(t * t * t * t - 1.0)
}

#[must_use]
pub fn quart_in_out(t: f32) -> f32 {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        let t = t - 1.0;
        -8.0 * t * t * t * t + 1.0
    }
}

#[must_use]
pub fn quint_in(t: f32) -> f32 {
    t * t * t * t * t
}

#[must_use]
pub fn quint_out(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t * t * t + 1.0
}

#[must_use]
pub fn quint_in_out(t: f32) -> f32 {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        let t = 2.0 * t - 2.0;
        0.5 * t * t * t * t * t + 1.0
    }
}

#[must_use]
pub fn sine_in(t: f32) -> f32 {
    (1.0 - (t * PI / 2.0).cos()).clamp(0.0, 1.0)
}

#[must_use]
pub fn sine_out(t: f32) -> f32 {
    (t * PI / 2.0).sin()
}

#[must_use]
pub fn sine_in_out(t: f32) -> f32 {
    ((t * PI).cos() - 1.0) / -2.0
}

#[must_use]
pub fn expo_in(t: f32) -> f32 {
    if t <= 0.0 {
        0.0
    } else {
        (2.0_f32).powf(10.0 * (t - 1.0))
    }
}

#[must_use]
pub fn expo_out(t: f32) -> f32 {
    if t >= 1.0 {
        1.0
    } else {
        1.0 - (2.0_f32).powf(-10.0 * t)
    }
}

#[must_use]
pub fn expo_in_out(t: f32) -> f32 {
    if t <= 0.0 || t >= 1.0 {
        t
    } else if t < 0.5 {
        (2.0_f32).powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - (2.0_f32).powf(-20.0 * t + 10.0)) / 2.0
    }
}

#[must_use]
pub fn circ_in(t: f32) -> f32 {
    1.0 - (1.0 - t * t).sqrt()
}

#[must_use]
pub fn circ_out(t: f32) -> f32 {
    let t = t - 1.0;
    (1.0 - t * t).sqrt()
}

#[must_use]
pub fn circ_in_out(t: f32) -> f32 {
    if t < 0.5 {
        (1.0 - (1.0 - 4.0 * t * t).sqrt()) / 2.0
    } else {
        let t = 2.0 * t - 2.0;
        ((1.0 - t * t).sqrt() + 1.0) / 2.0
    }
}

const BACK_OVERSHOOT: f32 = 1.70158;
const BACK_OVERSHOOT_MORE: f32 = BACK_OVERSHOOT * 1.525;

#[must_use]
pub fn back_in(t: f32) -> f32 {
    t * t * ((BACK_OVERSHOOT + 1.0) * t - BACK_OVERSHOOT)
}

#[must_use]
pub fn back_out(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * ((BACK_OVERSHOOT + 1.0) * t + BACK_OVERSHOOT) + 1.0
}

#[must_use]
pub fn back_in_out(t: f32) -> f32 {
    if t < 0.5 {
        let t = 2.0 * t;
        0.5 * (t * t * ((BACK_OVERSHOOT_MORE + 1.0) * t - BACK_OVERSHOOT_MORE))
    } else {
        let t = 2.0 * t - 2.0;
        0.5 * (t * t * ((BACK_OVERSHOOT_MORE + 1.0) * t + BACK_OVERSHOOT_MORE) + 2.0)
    }
}

#[must_use]
pub fn elastic_in(t: f32) -> f32 {
    if t <= 0.0 || t >= 1.0 {
        t
    } else {
        -(2.0_f32).powf(10.0 * (t - 1.0)) * ((t - 1.075) * (2.0 * PI) / 0.3).sin()
    }
}

#[must_use]
pub fn elastic_out(t: f32) -> f32 {
    if t <= 0.0 || t >= 1.0 {
        t
    } else {
        (2.0_f32).powf(-10.0 * t) * ((t - 0.075) * (2.0 * PI) / 0.3).sin() + 1.0
    }
}

#[must_use]
pub fn elastic_in_out(t: f32) -> f32 {
    if t <= 0.0 || t >= 1.0 {
        t
    } else if t < 0.5 {
        -((2.0_f32).powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * (2.0 * PI) / 0.45).sin()) / 2.0
    } else {
        ((2.0_f32).powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * (2.0 * PI) / 0.45).sin()) / 2.0 + 1.0
    }
}

#[must_use]
pub fn bounce_in(t: f32) -> f32 {
    1.0 - bounce_out(1.0 - t)
}

#[must_use]
pub fn bounce_out(t: f32) -> f32 {
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        let t = t - 1.5 / 2.75;
        7.5625 * t * t + 0.75
    } else if t < 2.5 / 2.75 {
        let t = t - 2.25 / 2.75;
        7.5625 * t * t + 0.9375
    } else {
        let t = t - 2.625 / 2.75;
        7.5625 * t * t + 0.984_375
    }
}

#[must_use]
pub fn bounce_in_out(t: f32) -> f32 {
    if t < 0.5 {
        (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
    }
}

#[must_use]
pub fn cubic_bezier(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let t = sample_curve_x(t, x1, x2);
    sample_curve_y(t, y1, y2)
}

fn sample_curve_x(t: f32, x1: f32, x2: f32) -> f32 {
    calc_bezier_t(calc_bezier(t, x1, x2), t, x1, x2)
}

fn sample_curve_y(t: f32, y1: f32, y2: f32) -> f32 {
    calc_bezier(t, y1, y2)
}

fn calc_bezier(t: f32, a: f32, b: f32) -> f32 {
    let s = 1.0 - t;
    s * s * s * 0.0 + 3.0 * s * s * t * a + 3.0 * s * t * t * b + t * t * t * 1.0
}

fn calc_bezier_t(guess: f32, t: f32, x1: f32, x2: f32) -> f32 {
    let mut t_guess = guess;
    for _ in 0..8 {
        let x = calc_bezier(t_guess, x1, x2) - t;
        let derivative = 3.0 * (1.0 - t_guess) * (1.0 - t_guess) * x1
            + 6.0 * (1.0 - t_guess) * t_guess * (x2 - x1)
            + 3.0 * t_guess * t_guess * (1.0 - x2);
        if derivative.abs() < 0.00001 {
            break;
        }
        t_guess -= x / derivative;
    }
    t_guess.clamp(0.0, 1.0)
}
