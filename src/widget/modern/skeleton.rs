//! Skeleton / Loading shimmer pour SoryOS.

use crate::widget::container;
use crate::Element;

/// Skeleton loading placeholder.
pub struct ModernSkeleton {
    width: Option<f32>,
    height: Option<f32>,
    rounded: bool,
    lines: usize,
}

impl ModernSkeleton {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            rounded: false,
            lines: 1,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }

    pub fn lines(mut self, lines: usize) -> Self {
        self.lines = lines;
        self
    }
}

impl<'a, Message: 'static> From<ModernSkeleton> for Element<'a, Message> {
    fn from(sk: ModernSkeleton) -> Self {
        use crate::iced::Length;
        use crate::widget::column;

        let mut col = column::with_capacity(sk.lines);
        for _ in 0..sk.lines {
            let w = sk.width.unwrap_or(100.0);
            let h = sk.height.unwrap_or(12.0);
            let placeholder = container(crate::widget::text::body(" "))
                .width(Length::Fixed(w))
                .height(Length::Fixed(h))
                .class(crate::theme::Container::Tooltip);
            col = col.push(placeholder);
        }
        col.into()
    }
}
