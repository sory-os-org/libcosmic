// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Onglets stylisés SoryOS avec indicateur glow.
//!
//! Fournit un composant de tabulation horizontal avec :
//! - Indicateur de underliner glow bleu sous l'onglet actif
//! - États normal / survolé / actif
//! - Support d'icônes dans les onglets
//! - Badge de notification sur les onglets

use std::borrow::Cow;

use crate::iced::{Alignment, Length, Padding};
use crate::widget::{column, container, mouse_area, row, space, text};
use crate::Element;

// ═════════════════════════════════════════════════════════════════════════════
// TABS
// ═════════════════════════════════════════════════════════════════════════════

/// Builder pour une barre d'onglets SoryOS.
pub struct SoryTabs<'a, Message> {
    tabs: Vec<SoryTab<'a, Message>>,
    active_index: usize,
    style: SoryTabStyle,
}

/// Style de la barre d'onglets.
pub enum SoryTabStyle {
    /// Style underline (indicateur sous l'onglet actif).
    Underline,
    /// Style pill (fond coloré).
    Pill,
    /// Style bordure (bordure gauche).
    BorderLeft,
}

struct SoryTab<'a, Message> {
    label: Cow<'a, str>,
    icon: Option<Element<'a, Message>>,
    badge: Option<Element<'a, Message>>,
    on_press: Option<Message>,
}

impl<'a, Message> Default for SoryTabs<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> SoryTabs<'a, Message> {
    /// Crée une nouvelle barre d'onglets.
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_index: 0,
            style: SoryTabStyle::Underline,
        }
    }

    /// Définit le style des onglets.
    pub fn style(mut self, style: SoryTabStyle) -> Self {
        self.style = style;
        self
    }

    /// Définit l'index de l'onglet actif.
    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }

    /// Ajoute un onglet.
    pub fn tab(mut self, label: impl Into<Cow<'a, str>>, on_press: Message) -> Self {
        self.tabs.push(SoryTab {
            label: label.into(),
            icon: None,
            badge: None,
            on_press: Some(on_press),
        });
        self
    }

    /// Ajoute un onglet avec icône.
    pub fn tab_with_icon(
        mut self,
        label: impl Into<Cow<'a, str>>,
        icon: impl Into<Element<'a, Message>>,
        on_press: Message,
    ) -> Self {
        self.tabs.push(SoryTab {
            label: label.into(),
            icon: Some(icon.into()),
            badge: None,
            on_press: Some(on_press),
        });
        self
    }

    /// Ajoute un badge à l'onglet suivant.
    pub fn badge(mut self, badge: impl Into<Element<'a, Message>>) -> Self {
        if let Some(tab) = self.tabs.last_mut() {
            tab.badge = Some(badge.into());
        }
        self
    }
}

impl<'a, Message: Clone + 'static> From<SoryTabs<'a, Message>> for Element<'a, Message> {
    fn from(tabs: SoryTabs<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        match tabs.style {
            SoryTabStyle::Underline => {
                // Conteneur de la barre d'onglets
                let mut tab_row = row::with_capacity(tabs.tabs.len())
                    .spacing(0)
                    .align_y(Alignment::End);

                for (i, tab) in tabs.tabs.into_iter().enumerate() {
                    let is_active = i == tabs.active_index;

                    let mut content = row::with_capacity(3)
                        .spacing(spacing.space_xxs)
                        .align_y(Alignment::Center);

                    if let Some(icon) = tab.icon {
                        content = content.push(icon);
                    }

                    content = content.push(text::body(tab.label));

                    if let Some(badge) = tab.badge {
                        content = content.push(badge);
                    }

                    let tab_item = container(content)
                        .padding(Padding::from([
                            spacing.space_xxs,
                            spacing.space_s,
                            0,
                            spacing.space_s,
                        ]))
                        .width(Length::Shrink);

                    // Indicateur underline pour l'onglet actif
                    let underline = if is_active {
                        Some(
                            container(space::horizontal())
                                .height(2.0)
                                .width(Length::Fill)
                                .class(crate::theme::sory::sidebar_accent_bar()),
                        )
                    } else {
                        None
                    };

                    let mut tab_col = column::with_capacity(2)
                        .push(tab_item)
                        .align_x(Alignment::Center);

                    if let Some(ul) = underline {
                        tab_col = tab_col.push(ul);
                    } else {
                        tab_col = tab_col.push(
                            container(space::horizontal())
                                .height(2.0)
                                .width(Length::Fill)
                                .class(crate::theme::sory::tab_underline_inactive()),
                        );
                    }

                    let tab_container = container(tab_col)
                        .width(Length::Shrink)
                        .padding(Padding::from([0, spacing.space_xxs]));

                    if let Some(on_press) = tab.on_press {
                        tab_row = tab_row.push(
                            mouse_area(tab_container).on_press(on_press),
                        );
                    } else {
                        tab_row = tab_row.push(tab_container);
                    }
                }

                // Fond de la barre d'onglets avec bordure inférieure
                container(tab_row)
                    .class(crate::theme::sory::tab_bar())
                    .width(Length::Fill)
                    .into()
            }
            SoryTabStyle::Pill => {
                let mut tab_row = row::with_capacity(tabs.tabs.len())
                    .spacing(spacing.space_xxs)
                    .align_y(Alignment::Center);

                for (i, tab) in tabs.tabs.into_iter().enumerate() {
                    let is_active = i == tabs.active_index;

                    let mut content = row::with_capacity(3)
                        .spacing(spacing.space_xxs)
                        .align_y(Alignment::Center);

                    if let Some(icon) = tab.icon {
                        content = content.push(icon);
                    }

                    content = content.push(text::body(tab.label));

                    if let Some(badge) = tab.badge {
                        content = content.push(badge);
                    }

                    let tab_class = if is_active {
                        crate::theme::sory::view_button_active()
                    } else {
                        crate::theme::sory::view_button()
                    };

                    let tab_container = container(content)
                        .class(tab_class)
                        .padding(Padding::from([
                            spacing.space_xxs,
                            spacing.space_s,
                        ]));

                    if let Some(on_press) = tab.on_press {
                        tab_row = tab_row.push(
                            mouse_area(tab_container).on_press(on_press),
                        );
                    } else {
                        tab_row = tab_row.push(tab_container);
                    }
                }

                container(tab_row)
                    .padding(spacing.space_xxs)
                    .width(Length::Fill)
                    .into()
            }
            SoryTabStyle::BorderLeft => {
                let mut tab_col = column::with_capacity(tabs.tabs.len())
                    .spacing(spacing.space_xxs);

                for (i, tab) in tabs.tabs.into_iter().enumerate() {
                    let is_active = i == tabs.active_index;

                    let mut content = row::with_capacity(3)
                        .spacing(spacing.space_s)
                        .align_y(Alignment::Center);

                    if let Some(icon) = tab.icon {
                        content = content.push(icon);
                    }

                    content = content.push(text::body(tab.label));

                    if let Some(badge) = tab.badge {
                        content = content.push(badge);
                    }

                    let border_bar = if is_active {
                        Some(
                            container(space::vertical())
                                .width(3.0)
                                .height(Length::Fill)
                                .class(crate::theme::sory::sidebar_accent_bar()),
                        )
                    } else {
                        None
                    };

                    let mut row_content = row::with_capacity(2)
                        .spacing(spacing.space_s)
                        .align_y(Alignment::Center);

                    if let Some(bar) = border_bar {
                        row_content = row_content.push(bar);
                    } else {
                        row_content = row_content.push(space::horizontal().width(3.0));
                    }

                    row_content = row_content.push(content.width(Length::Fill));

                    let tab_container = container(row_content)
                        .padding(Padding::from([
                            spacing.space_s,
                            spacing.space_s,
                        ]))
                        .width(Length::Fill);

                    if let Some(on_press) = tab.on_press {
                        tab_col = tab_col.push(
                            mouse_area(tab_container).on_press(on_press),
                        );
                    } else {
                        tab_col = tab_col.push(tab_container);
                    }
                }

                container(tab_col)
                    .class(crate::theme::sory::sidebar())
                    .width(Length::Shrink)
                    .into()
            }
        }
    }
}

/// Crée une barre d'onglets simple avec des labels textuels.
pub fn simple_tabs<'a, Message: Clone + 'static>(
    labels: &[&'a str],
    active: usize,
    on_select: impl Fn(usize) -> Message,
) -> Element<'a, Message> {
    let mut tabs = SoryTabs::new().active(active);
    for (i, label) in labels.iter().enumerate() {
        tabs = tabs.tab(*label, on_select(i));
    }
    tabs.into()
}
