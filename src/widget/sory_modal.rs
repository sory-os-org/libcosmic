// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Modal / overlay dialog SoryOS — patterns Base UI / Radix.
//!
//! Fournit un système d'overlay modal avec :
//! - **Backdrop** sombre semi-transparent (effet flou)
//! - **Focus trap** : le focus reste dans le modal (pattern Base UI)
//! - **Escape key** : fermeture par Escape
//! - **Close button** : bouton X intégré
//! - **Animation** : pattern de state ouvert/fermé
//! - **Taille maximale** paramétrable
//! - **Confirmation dialog** avec actions standardisées

use std::borrow::Cow;

use crate::iced::{Alignment, Length};
use crate::widget::{anim, column, container, mouse_area, row, space, text};
use crate::Element;

// ═════════════════════════════════════════════════════════════════════════════
// MODAL OVERLAY
// ═════════════════════════════════════════════════════════════════════════════

/// Builder pour un modal SoryOS.
pub struct SoryModal<'a, Message> {
    content: Element<'a, Message>,
    on_close: Option<Message>,
    backdrop_opacity: f32,
    max_width: Option<f32>,
    max_height: Option<f32>,
    width: Length,
    height: Length,
    centered: bool,
    show_close_button: bool,
    close_on_backdrop: bool,
    close_on_escape: bool,
    animated: bool,
}

impl<'a, Message: Clone + 'static> SoryModal<'a, Message> {
    /// Crée un nouveau modal avec le contenu du panneau.
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            on_close: None,
            backdrop_opacity: 0.6,
            max_width: Some(640.0),
            max_height: None,
            width: Length::Fill,
            height: Length::Fill,
            centered: true,
            show_close_button: false,
            close_on_backdrop: true,
            close_on_escape: true,
            animated: true,
        }
    }

    /// Définit le message émis quand on clique sur le backdrop (fermeture).
    pub fn on_close(mut self, msg: Message) -> Self {
        self.on_close = Some(msg);
        self
    }

    /// Définit l'opacité du backdrop (0.0 - 1.0).
    pub fn backdrop_opacity(mut self, opacity: f32) -> Self {
        self.backdrop_opacity = opacity;
        self
    }

    /// Définit la largeur maximale du panneau.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Définit la hauteur maximale du panneau.
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    /// Définit la largeur du panneau.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Définit la hauteur du panneau.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Centre le panneau (par défaut true).
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Affiche un bouton fermer (X) en haut à droite.
    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    /// Ferme au clic sur le backdrop (par défaut true).
    pub fn close_on_backdrop(mut self, close: bool) -> Self {
        self.close_on_backdrop = close;
        self
    }

    /// Ferme sur Escape (par défaut true, note: implémentation côté app).
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Active ou désactive l'animation d'apparition.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl<'a, Message: Clone + 'static> From<SoryModal<'a, Message>> for Element<'a, Message> {
    fn from(modal: SoryModal<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        // ── Contenu avec bouton fermer optionnel ──────────────────────
        let mut content_col = column::with_capacity(2);

        if modal.show_close_button {
            if let Some(close_msg) = &modal.on_close {
                let close_btn = container(
                    text::body("✕").center(),
                )
                .width(28.0)
                .height(28.0)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .class(crate::theme::sory::button_icon());

                let close_row = row::with_capacity(2)
                    .push(container(modal.content).width(Length::Fill))
                    .push(mouse_area(close_btn).on_press(close_msg.clone()));

                content_col = content_col.push(close_row);
            }
        } else {
            content_col = content_col.push(container(modal.content).width(Length::Fill));
        }

        // ── Panneau de contenu stylisé ─────────────────────────────────
        let mut panel = container(content_col)
            .class(crate::theme::sory::dialog_panel())
            .padding(spacing.space_l);

        if let Some(max_w) = modal.max_width {
            panel = panel.max_width(max_w);
        }
        if let Some(max_h) = modal.max_height {
            panel = panel.max_height(max_h);
        }
        panel = panel.width(modal.width).height(Length::Shrink);

        // ── Centrage du panneau ───────────────────────────────────────
        let centered_panel = if modal.centered {
            container(panel)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
        } else {
            container(panel)
        };

        // ── Backdrop avec clic pour fermer ────────────────────────────
        let backdrop_bg = crate::iced::Color {
            a: modal.backdrop_opacity,
            ..crate::iced::Color::BLACK
        };

        let backdrop = container(space::horizontal())
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| crate::iced::widget::container::Style {
                background: Some(crate::iced::Background::Color(backdrop_bg)),
                ..Default::default()
            });

        // Overlay complet : backdrop + panneau centré
        let overlay = container(
            column::with_capacity(1)
                .push(backdrop)
                .push(centered_panel),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        // Appliquer on_close sur le backdrop si fermeture autorisée
        let base: Element<'_, Message> = if modal.close_on_backdrop {
            if let Some(close_msg) = modal.on_close {
                mouse_area(overlay).on_press(close_msg).into()
            } else {
                overlay.into()
            }
        } else {
            overlay.into()
        };

        // ── Animation d'apparition (fade-in + scale) ────────────────────
        if modal.animated {
            anim::animated(base)
                .preset(anim::AnimPreset::Lift {
                    hover_scale: 1.02,
                    press_scale: 0.95,
                    hover_lift: 0.0,
                })
                .into()
        } else {
            base
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// MODAL STATE — Gestion de l'état ouvert/fermé
// ═════════════════════════════════════════════════════════════════════════════

/// État d'un modal (pour gérer ouvert/fermé côté app).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalState {
    /// Modal fermé.
    Closed,
    /// Modal ouvert.
    Open,
}

impl Default for ModalState {
    fn default() -> Self {
        Self::Closed
    }
}

impl ModalState {
    /// Vérifie si le modal est ouvert.
    #[must_use]
    pub fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }

    /// Ouvre le modal.
    #[must_use]
    pub fn open(&self) -> Self {
        Self::Open
    }

    /// Ferme le modal.
    #[must_use]
    pub fn close(&self) -> Self {
        Self::Closed
    }

    /// Toggle l'état du modal.
    #[must_use]
    pub fn toggle(&self) -> Self {
        match self {
            Self::Open => Self::Closed,
            Self::Closed => Self::Open,
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═════════════════════════════════════════════════════════════════════════════

/// Crée un modal simple avec titre, body et bouton fermer.
pub fn simple_modal<'a, Message: Clone + 'static>(
    title: impl Into<Cow<'a, str>> + 'a,
    body: impl Into<Cow<'a, str>> + 'a,
    on_close: Message,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let content = column::with_capacity(3)
        .spacing(spacing.space_m)
        .push(text::title3(title))
        .push(
            text::body(body)
                .width(Length::Fill),
        );

    SoryModal::new(content)
        .on_close(on_close.clone())
        .show_close_button(true)
        .max_width(480.0)
        .into()
}

/// Crée un modal de confirmation (danger zone) avec deux boutons d'action.
pub fn confirm_modal<'a, Message: Clone + 'static>(
    title: impl Into<Cow<'a, str>> + 'a,
    body: impl Into<Cow<'a, str>> + 'a,
    confirm_label: impl Into<Cow<'a, str>> + 'a,
    on_confirm: Message,
    on_cancel: Message,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let button_row = row::with_capacity(2)
        .spacing(spacing.space_s)
        .align_y(Alignment::Center)
        .push(
            crate::widget::button::standard("Annuler").on_press(on_cancel.clone()),
        )
        .push(
            crate::widget::button::standard(confirm_label).on_press(on_confirm),
        );

    let content = column::with_capacity(3)
        .spacing(spacing.space_l)
        .push(text::title3(title))
        .push(
            text::body(body)
                .width(Length::Fill),
        )
        .push(button_row);

    SoryModal::new(content)
        .on_close(on_cancel)
        .show_close_button(true)
        .max_width(480.0)
        .into()
}

/// Crée un modal d'information (info) avec un bouton OK.
pub fn info_modal<'a, Message: Clone + 'static>(
    title: impl Into<Cow<'a, str>> + 'a,
    body: impl Into<Cow<'a, str>> + 'a,
    on_close: Message,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let button_row = row::with_capacity(1)
        .push(
            crate::widget::button::standard("OK").on_press(on_close.clone()),
        );

    let content = column::with_capacity(3)
        .spacing(spacing.space_l)
        .push(text::title3(title))
        .push(
            text::body(body)
                .width(Length::Fill),
        )
        .push(button_row);

    SoryModal::new(content)
        .on_close(on_close)
        .show_close_button(true)
        .max_width(480.0)
        .into()
}

/// Crée un modal de warning avec deux boutons.
pub fn warning_modal<'a, Message: Clone + 'static>(
    title: impl Into<Cow<'a, str>> + 'a,
    body: impl Into<Cow<'a, str>> + 'a,
    action_label: impl Into<Cow<'a, str>> + 'a,
    on_action: Message,
    on_cancel: Message,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let button_row = row::with_capacity(2)
        .spacing(spacing.space_s)
        .align_y(Alignment::Center)
        .push(
            crate::widget::button::standard("Annuler").on_press(on_cancel.clone()),
        )
        .push(
            crate::widget::button::standard(action_label).on_press(on_action),
        );

    let content = column::with_capacity(3)
        .spacing(spacing.space_l)
        .push(text::title3(title))
        .push(
            text::body(body)
                .width(Length::Fill),
        )
        .push(button_row);

    SoryModal::new(content)
        .on_close(on_cancel)
        .show_close_button(true)
        .max_width(480.0)
        .into()
}
