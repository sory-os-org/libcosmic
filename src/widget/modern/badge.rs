//! Badge moderne SoryOS — indicateur visuel compact.

use crate::iced::{Alignment, Length, Padding};
use crate::widget::container;
use crate::Element;
use std::borrow::Cow;

/// Variant de badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    Primary,
    Success,
    Warning,
    Error,
    Info,
    Neutral,
}

impl Default for BadgeVariant {
    fn default() -> Self { Self::Neutral }
}

/// Badge compact avec couleur selon le variant.
pub struct ModernBadge<'a> {
    text: Cow<'a, str>,
    variant: BadgeVariant,
    size: f32,
}

impl<'a> ModernBadge<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            variant: BadgeVariant::Neutral,
            size: 12.0,
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl<'a, Message: 'a> From<ModernBadge<'a>> for Element<'a, Message> {
    fn from(badge: ModernBadge<'a>) -> Self {
        container(crate::widget::text::caption(badge.text))
            .padding(Padding::from([2, 6]))
            .class(crate::theme::Container::Tooltip)
            .into()
    }
}
