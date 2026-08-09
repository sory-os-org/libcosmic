//! Barre de progression moderne SoryOS avec animation spring.

use crate::anim::spring::{Spring, SpringConfig};
use crate::widget::progress_bar;
use crate::Element;
use std::time::Instant;

type SpringF32 = Spring<f32>;

/// Barre de progression moderne avec animation spring.
pub struct ModernProgressBar {
    progress: f32,
    animated: bool,
    height: f32,
    spring: Spring<f32>,
}

impl ModernProgressBar {
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            animated: true,
            height: 4.0,
            spring: SpringF32::new_with_config(0.0, SpringConfig::CRITICAL),
        }
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn tick(&mut self, now: Instant) {
        if self.animated {
            self.spring.set_target(self.progress);
            self.spring.update(now);
        }
    }
}
