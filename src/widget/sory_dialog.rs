// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Widget de dialogue SoryOS.
//!
//! `SoryDialog` est un builder qui assemble un panneau de dialogue stylisé
//! avec titre, icône, corps, contrôles et boutons d'action.

use std::borrow::Cow;

use crate::iced::core::text::Wrapping;
use crate::iced::{Alignment, Length, Pixels};
use crate::palette;
use crate::widget::{anim, column, container, row, space, text};
use crate::{Apply, Element, theme};

/// Construit un nouveau dialogue SoryOS.
#[must_use]
pub fn sory_dialog<'a, Message>() -> SoryDialog<'a, Message> {
    SoryDialog::new()
}

/// Builder pour un dialogue stylisé SoryOS.
pub struct SoryDialog<'a, Message> {
    title: Option<Cow<'a, str>>,
    icon: Option<Element<'a, Message>>,
    body: Option<Cow<'a, str>>,
    controls: Vec<Element<'a, Message>>,
    primary_action: Option<Element<'a, Message>>,
    secondary_action: Option<Element<'a, Message>>,
    tertiary_action: Option<Element<'a, Message>>,
    width: Option<Length>,
    height: Option<Length>,
    max_width: Option<Pixels>,
    max_height: Option<Pixels>,
    animated: bool,
}

impl<Message> Default for SoryDialog<'_, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> SoryDialog<'a, Message> {
    /// Crée un dialogue vide.
    pub fn new() -> Self {
        Self {
            title: None,
            icon: None,
            body: None,
            controls: Vec::new(),
            primary_action: None,
            secondary_action: None,
            tertiary_action: None,
            width: None,
            height: None,
            max_width: None,
            max_height: None,
            animated: true,
        }
    }

    /// Définit le titre du dialogue.
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Définit l'icône du dialogue.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Définit le texte du corps.
    pub fn body(mut self, body: impl Into<Cow<'a, str>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Ajoute un contrôle (input, toggle, etc.) au dialogue.
    pub fn control(mut self, control: impl Into<Element<'a, Message>>) -> Self {
        self.controls.push(control.into());
        self
    }

    /// Définit le bouton d'action principale.
    pub fn primary_action(mut self, button: impl Into<Element<'a, Message>>) -> Self {
        self.primary_action = Some(button.into());
        self
    }

    /// Définit le bouton d'action secondaire.
    pub fn secondary_action(mut self, button: impl Into<Element<'a, Message>>) -> Self {
        self.secondary_action = Some(button.into());
        self
    }

    /// Définit le bouton d'action tertiaire.
    pub fn tertiary_action(mut self, button: impl Into<Element<'a, Message>>) -> Self {
        self.tertiary_action = Some(button.into());
        self
    }

    /// Définit la largeur du dialogue.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Définit la hauteur du dialogue.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Définit la largeur maximale du dialogue.
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = Some(max_width.into());
        self
    }

    /// Définit la hauteur maximale du dialogue.
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = Some(max_height.into());
        self
    }

    /// Active ou désactive l'animation d'apparition (fade-in).
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl<'a, Message: Clone + 'static> From<SoryDialog<'a, Message>> for Element<'a, Message> {
    fn from(dialog: SoryDialog<'a, Message>) -> Self {
        let spacing = theme::spacing();

        // ── Colonne de contenu ────────────────────────────────────────────
        let mut content_col = column::with_capacity(3 + dialog.controls.len() * 2);
        let mut should_space = false;

        if let Some(title) = dialog.title {
            content_col = content_col.push(text::title3(title));
            should_space = true;
        }

        if let Some(body) = dialog.body {
            if should_space {
                    content_col =
                    content_col.push(space::vertical().height(Length::Fixed(spacing.space_xxs.into())));

            }
            content_col = content_col
                .push(container(text::body(body).wrapping(Wrapping::Word)).max_height(260.0));
            should_space = true;
        }

        for control in dialog.controls {
            if should_space {
                content_col =
                    content_col.push(space::vertical().height(Length::Fixed(spacing.space_s.into())));
            }
            content_col = content_col.push(
                container(control)
                    .class(crate::theme::sory::context_content())
                    .padding(10)
                    .width(Length::Fill),
            );
            should_space = true;
        }

        // ── Ligne icône + contenu ────────────────────────────────────────
        let mut content_row = row::with_capacity(2)
            .spacing(spacing.space_s)
            .align_y(Alignment::Start);

        if let Some(icon) = dialog.icon {
            content_row = content_row.push(
                container(icon)
                    .class(crate::theme::sory::tile_icon_blue())
                    .padding(12),
            );
        }
        content_row = content_row.push(container(content_col).width(Length::Fill));

        // ── Ligne de boutons ─────────────────────────────────────────────
        let mut button_row = row::with_capacity(4)
            .spacing(spacing.space_xxs)
            .align_y(Alignment::Center);

        if let Some(button) = dialog.tertiary_action {
            button_row = button_row.push(button);
        }
        button_row = button_row.push(space::horizontal());
        if let Some(button) = dialog.secondary_action {
            button_row = button_row.push(button);
        }
        if let Some(button) = dialog.primary_action {
            button_row = button_row.push(button);
        }

        // ── Assemblage du panneau ────────────────────────────────────────
        let mut panel = container(
            column::with_children([content_row.into(), button_row.into()]).spacing(spacing.space_l),
        )
        .class(crate::theme::sory::dialog_panel())
        .padding(spacing.space_m)
        .width(dialog.width.unwrap_or(Length::Fixed(570.0)));

        if let Some(height) = dialog.height {
            panel = panel.height(height);
        }
        if let Some(max_width) = dialog.max_width {
            panel = panel.max_width(max_width);
        }
        if let Some(max_height) = dialog.max_height {
            panel = panel.max_height(max_height);
        }

        let mut base: Element<'_, Message> = panel.into();

        if dialog.animated {
            base = anim::animated(base)
                .preset(anim::AnimPreset::Lift {
                    hover_scale: 1.0,
                    press_scale: 0.98,
                    hover_lift: 0.0,
                })
                .into();
        }

        base
    }
}
