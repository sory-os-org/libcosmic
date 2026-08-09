use std::time::{Duration, Instant};

use iced::Color;
use iced_core::{Point, Size, Vector};

use super::easing::Easing;

pub trait Interpolate: Copy {
    fn lerp(self, other: Self, t: f32) -> Self;
}

impl Interpolate for f32 {
    fn lerp(self, other: Self, t: f32) -> Self {
        (1.0 - t) * self + t * other
    }
}

impl Interpolate for f64 {
    fn lerp(self, other: Self, t: f32) -> Self {
        (1.0 - t as f64) * self + t as f64 * other
    }
}

impl Interpolate for u8 {
    fn lerp(self, other: Self, t: f32) -> Self {
        (self as f32 + (other as f32 - self as f32) * t).round() as u8
    }
}

impl Interpolate for u16 {
    fn lerp(self, other: Self, t: f32) -> Self {
        (self as f32 + (other as f32 - self as f32) * t).round() as u16
    }
}

impl Interpolate for u32 {
    fn lerp(self, other: Self, t: f32) -> Self {
        (self as f32 + (other as f32 - self as f32) * t).round() as u32
    }
}

impl Interpolate for i32 {
    fn lerp(self, other: Self, t: f32) -> Self {
        (self as f32 + (other as f32 - self as f32) * t).round() as i32
    }
}

impl Interpolate for bool {
    fn lerp(self, other: Self, t: f32) -> Self {
        if t < 0.5 { self } else { other }
    }
}

impl Interpolate for Point {
    fn lerp(self, other: Self, t: f32) -> Self {
        Point::new(self.x.lerp(other.x, t), self.y.lerp(other.y, t))
    }
}

impl Interpolate for Size {
    fn lerp(self, other: Self, t: f32) -> Self {
        Size::new(self.width.lerp(other.width, t), self.height.lerp(other.height, t))
    }
}

impl Interpolate for Vector {
    fn lerp(self, other: Self, t: f32) -> Self {
        Vector::new(self.x.lerp(other.x, t), self.y.lerp(other.y, t))
    }
}

impl Interpolate for Color {
    fn lerp(self, other: Self, t: f32) -> Self {
        Color {
            r: self.r.lerp(other.r, t),
            g: self.g.lerp(other.g, t),
            b: self.b.lerp(other.b, t),
            a: self.a.lerp(other.a, t),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Eased<T: Interpolate> {
    Idle(T),
    Animating {
        from: T,
        to: T,
        start: Instant,
        duration: Duration,
        easing: Easing,
    },
}

impl<T: Interpolate> Eased<T> {
    pub fn new(value: T) -> Self {
        Self::Idle(value)
    }

    pub fn animate_to(&mut self, to: T, duration: Duration, easing: Easing) {
        let from = self.value();
        *self = Self::Animating {
            from,
            to,
            start: Instant::now(),
            duration,
            easing,
        };
    }

    pub fn value(&self) -> T {
        match self {
            Self::Idle(v) => *v,
            Self::Animating { from, to, start, duration, easing } => {
                let elapsed = Instant::now().duration_since(*start);
                let t = (elapsed.as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0);
                let eased = easing.apply(t);
                from.lerp(*to, eased)
            }
        }
    }

    pub fn update(&mut self, now: Instant) -> bool {
        match self {
            Self::Idle(_) => false,
            Self::Animating { from: _, to, start, duration, .. } => {
                let elapsed = now.duration_since(*start);
                if elapsed >= *duration {
                    *self = Self::Idle(*to);
                    return false;
                }
                true
            }
        }
    }

    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle(_))
    }

    pub fn is_animating(&self) -> bool {
        matches!(self, Self::Animating { .. })
    }
}
