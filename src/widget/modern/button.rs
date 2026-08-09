//! Bouton moderne SoryOS avec micro-interactions (hover scale, press scale).
//!
//! Utilise `crate::widget::anim::Animated` pour les animations spring physiques
//! et les design tokens pour le style cohérent.

use crate::iced::{Alignment, Length, Padding};
use crate::widget::anim::{self, AnimPreset};
use crate::widget::{button, container, mouse_area, row, text};
use crate::Element;
use std::borrow::Cow;

/// Variant de style pour le bouton moderne.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Plein,
    Outline,
    Ghost,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Plein
    }
}

/// Bouton moderne avec animation hover/press intégrée.
pub struct ModernButton<'a, Message> {
    label: Cow<'a, str>,
    variant: ButtonVariant,
    on_press: Option<Message>,
    width: Length,
    height: Length,
    padding: Padding,
    icon: Option<Element<'a, Message>>,
}

impl<'a, Message: Clone + 'static> ModernButton<'a, Message> {
    pub fn new(label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Plein,
            on_press: None,
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::from([10, 20]),
            icon: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl<'a, Message: Clone + 'static> From<ModernButton<'a, Message>> for Element<'a, Message> {
    fn from(btn: ModernButton<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        let mut content = row::with_capacity(2)
            .align_y(Alignment::Center)
            .spacing(spacing.space_xs);

        if let Some(icon) = btn.icon {
            content = content.push(icon);
        }

        content = content.push(text::body(btn.label));

        let mut btn_widget = button::custom(content)
            .padding(btn.padding)
            .width(btn.width)
            .height(btn.height);

        btn_widget = match btn.variant {
            ButtonVariant::Plein => btn_widget.class(crate::theme::Button::Suggested),
            ButtonVariant::Outline => btn_widget.class(crate::theme::Button::Standard),
            ButtonVariant::Ghost => btn_widget.class(crate::theme::Button::Text),
        };

        if let Some(on_press) = btn.on_press {
            btn_widget = btn_widget.on_press(on_press);
        }

        let base: Element<'_, Message> = btn_widget.into();

        anim::animated(base)
            .preset(AnimPreset::Lift {
                hover_scale: 1.03,
                press_scale: 0.97,
                hover_lift: -1.0,
            })
            .into()
    }
}
