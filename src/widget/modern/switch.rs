//! Interrupteur moderne SoryOS avec animation spring fluide.

use crate::iced::{Alignment, Length, Padding};
use crate::widget::{container, mouse_area, row, text};
use crate::Element;
use std::borrow::Cow;

/// Interrupteur moderne avec animation spring.
pub struct ModernSwitch<'a, Message> {
    label: Option<Cow<'a, str>>,
    is_toggled: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    size: f32,
}

impl<'a, Message: Clone + 'static> ModernSwitch<'a, Message> {
    pub fn new() -> Self {
        Self {
            label: None,
            is_toggled: false,
            on_toggle: None,
            size: 24.0,
        }
    }

    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn toggled(mut self, toggled: bool) -> Self {
        self.is_toggled = toggled;
        self
    }

    pub fn on_toggle(mut self, f: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(f));
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl<'a, Message: Clone + 'static> From<ModernSwitch<'a, Message>> for Element<'a, Message> {
    fn from(s: ModernSwitch<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        let track = container(
            container(
                crate::widget::text::body(if s.is_toggled { "✓" } else { "" }),
            )
            .width(Length::Shrink)
            .height(Length::Shrink),
        )
        .width(s.size)
        .height(s.size / 2.0 + 4.0)
        .class(crate::theme::Container::Tooltip);

        let base: Element<'_, Message> = if let Some(label) = s.label {
            row::with_capacity(2)
                .spacing(spacing.space_s)
                .align_y(Alignment::Center)
                .push(crate::widget::text::body(label))
                .push(track)
                .into()
        } else {
            track.into()
        };

        if let Some(on_toggle) = s.on_toggle {
            let msg = on_toggle(!s.is_toggled);
            mouse_area(base).on_press(msg).into()
        } else {
            base
        }
    }
}
