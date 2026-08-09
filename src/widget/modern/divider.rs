//! Diviseur moderne SoryOS.

use crate::widget::{container, space};
use crate::Element;

/// Diviseur moderne (ligne horizontale ou verticale).
pub struct ModernDivider {
    vertical: bool,
    spacing: f32,
}

impl ModernDivider {
    pub fn new() -> Self {
        Self {
            vertical: false,
            spacing: 8.0,
        }
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<'a, Message: 'static> From<ModernDivider> for Element<'a, Message> {
    fn from(d: ModernDivider) -> Self {
        let spacing = crate::theme::spacing();
        let s = spacing.space_s as f32;

        if d.vertical {
            container(space::vertical())
                .width(1.0)
                .height(d.spacing)
                .class(crate::theme::Container::Tooltip)
                .into()
        } else {
            container(space::horizontal())
                .width(d.spacing)
                .height(1.0)
                .class(crate::theme::Container::Tooltip)
                .into()
        }
    }
}
