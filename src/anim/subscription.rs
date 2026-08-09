use std::time::{Duration, Instant};

pub fn animation_frame() -> iced::Subscription<Instant> {
    iced::time::every(Duration::from_millis(16))
}

pub fn animation_frame_at(fps: u32) -> iced::Subscription<Instant> {
    let period = Duration::from_millis((1000.0 / fps as f32).max(1.0) as u64);
    iced::time::every(period)
}

pub fn animation_frame_every(duration: Duration) -> iced::Subscription<Instant> {
    iced::time::every(duration)
}
