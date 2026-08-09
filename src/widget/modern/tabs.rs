//! Tabs modernes SoryOS.

use crate::iced::{Alignment, Length, Padding};
use crate::widget::{column, container, mouse_area, row, space, text};
use crate::Element;
use std::borrow::Cow;

/// Un onglet.
pub struct Tab<'a, Message> {
    label: Cow<'a, str>,
    icon: Option<Element<'a, Message>>,
    on_press: Message,
    active: bool,
    badge: Option<Cow<'a, str>>,
}

/// Variants de style pour les tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStyle {
    Underline,
    Pill,
    Enclosed,
    Soft,
}

impl Default for TabStyle {
    fn default() -> Self { Self::Underline }
}

/// Tabs modernes avec support underline, pill, enclosed.
pub struct ModernTabs<'a, Message> {
    tabs: Vec<Tab<'a, Message>>,
    style: TabStyle,
    fill: bool,
}

impl<'a, Message: Clone + 'static> ModernTabs<'a, Message> {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            style: TabStyle::Underline,
            fill: false,
        }
    }

    pub fn style(mut self, style: TabStyle) -> Self {
        self.style = style;
        self
    }

    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    pub fn push_tab(
        mut self,
        label: impl Into<Cow<'a, str>>,
        active: bool,
        on_press: Message,
    ) -> Self {
        self.tabs.push(Tab {
            label: label.into(),
            icon: None,
            on_press,
            active,
            badge: None,
        });
        self
    }

    pub fn push_tab_with_icon(
        mut self,
        label: impl Into<Cow<'a, str>>,
        icon: Element<'a, Message>,
        active: bool,
        on_press: Message,
    ) -> Self {
        self.tabs.push(Tab {
            label: label.into(),
            icon: Some(icon),
            on_press,
            active,
            badge: None,
        });
        self
    }

    pub fn push_tab_with_badge(
        mut self,
        label: impl Into<Cow<'a, str>>,
        badge: impl Into<Cow<'a, str>>,
        active: bool,
        on_press: Message,
    ) -> Self {
        self.tabs.push(Tab {
            label: label.into(),
            icon: None,
            on_press,
            active,
            badge: Some(badge.into()),
        });
        self
    }
}

impl<'a, Message: Clone + 'static> From<ModernTabs<'a, Message>> for Element<'a, Message> {
    fn from(tabs: ModernTabs<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        let mut row = row::with_capacity(tabs.tabs.len())
            .spacing(spacing.space_xxs)
            .align_y(Alignment::Center);

        for tab in tabs.tabs {
            let mut content = row::with_capacity(2)
                .spacing(spacing.space_xxs)
                .align_y(Alignment::Center);

            if let Some(icon) = tab.icon {
                content = content.push(icon);
            }
            let label_str = tab.label.into_owned();
            content = content.push(text::body(label_str));

            if let Some(badge) = tab.badge {
                let badge_element: Element<'_, Message> = crate::widget::modern::ModernBadge::new(badge).into();
                content = content.push(badge_element);
            }

            let tab_element: Element<'_, Message> = if tab.active {
                container(content)
                    .padding(Padding::from([spacing.space_xxs, spacing.space_s]))
                    .width(if tabs.fill { Length::Fill } else { Length::Shrink })
                    .class(crate::theme::Container::Card)
                    .into()
            } else {
                container(content)
                    .padding(Padding::from([spacing.space_xxs, spacing.space_s]))
                    .width(if tabs.fill { Length::Fill } else { Length::Shrink })
                    .into()
            };

            row = row.push(
                mouse_area(tab_element)
                    .on_press(tab.on_press),
            );
        }

        row.into()
    }
}
