// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Système de toast / notification SoryOS — pattern shadcn Sonner.
//!
//! Fournit des notifications temporelles avec :
//! - **Auto-dismiss** : fermeture automatique après un délai
//! - **Variants** : success, error, warning, info
//! - **Actions** : bouton d'action optionnel
//! - **Stacking** : plusieurs toasts empilés
//! - **Position** : coins de l'écran (top-right, bottom-right, etc.)

use std::borrow::Cow;

use crate::iced::{Alignment, Length, Padding};
use crate::widget::{anim, column, container, mouse_area, row, space, text};
use crate::Element;

// ═════════════════════════════════════════════════════════════════════════════
// TOAST VARIANT
// ═════════════════════════════════════════════════════════════════════════════

/// Variants de toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastVariant {
    /// Information (bleu).
    Info,
    /// Succès (vert).
    Success,
    /// Avertissement (orange).
    Warning,
    /// Erreur (rouge).
    Error,
}

impl Default for ToastVariant {
    fn default() -> Self {
        Self::Info
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TOAST POSITION
// ═════════════════════════════════════════════════════════════════════════════

/// Position du toast à l'écran.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastPosition {
    /// En haut à droite.
    TopRight,
    /// En haut à gauche.
    TopLeft,
    /// En haut au centre.
    TopCenter,
    /// En bas à droite.
    BottomRight,
    /// En bas à gauche.
    BottomLeft,
    /// En bas au centre.
    BottomCenter,
}

impl Default for ToastPosition {
    fn default() -> Self {
        Self::BottomRight
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TOAST ITEM
// ═════════════════════════════════════════════════════════════════════════════

/// Un toast individuel.
#[derive(Clone)]
pub struct ToastItem<'a, Message> {
    id: ToastId,
    variant: ToastVariant,
    title: Cow<'a, str>,
    description: Option<Cow<'a, str>>,
    action: Option<ToastAction<'a, Message>>,
    dismissible: bool,
}

/// ID unique d'un toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToastId(pub usize);

/// Action du toast (bouton).
#[derive(Clone)]
pub struct ToastAction<'a, Message> {
    pub label: Cow<'a, str>,
    pub on_press: Message,
}

impl<'a, Message: Clone + 'static> ToastItem<'a, Message> {
    /// Crée un nouveau toast.
    pub fn new(id: usize, variant: ToastVariant, title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            id: ToastId(id),
            variant,
            title: title.into(),
            description: None,
            action: None,
            dismissible: true,
        }
    }

    /// Ajoute une description.
    pub fn description(mut self, desc: impl Into<Cow<'a, str>>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Ajoute une action (bouton).
    pub fn action(mut self, label: impl Into<Cow<'a, str>>, on_press: Message) -> Self {
        self.action = Some(ToastAction {
            label: label.into(),
            on_press,
        });
        self
    }

    /// Rend le toast non dismissible (pas de bouton X).
    pub fn not_dismissible(mut self) -> Self {
        self.dismissible = false;
        self
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TOAST CONTAINER (wrapper pour un toast)
// ═════════════════════════════════════════════════════════════════════════════

/// Crée le contenu d'un toast stylisé.
pub fn toast_content<Message: Clone + 'static>(
    toast: ToastItem<'static, Message>,
    on_dismiss: Option<Message>,
) -> Element<'static, Message> {
    let spacing = crate::theme::spacing();

    // Style selon le variant
    let toast_class = match toast.variant {
        ToastVariant::Info => crate::theme::sory::toast_info(),
        ToastVariant::Success => crate::theme::sory::toast_success(),
        ToastVariant::Warning => crate::theme::sory::toast_warning(),
        ToastVariant::Error => crate::theme::sory::toast_error(),
    };

    // Icône selon le variant
    let icon_char = match toast.variant {
        ToastVariant::Info => "ℹ",
        ToastVariant::Success => "✓",
        ToastVariant::Warning => "⚠",
        ToastVariant::Error => "✕",
    };

    // Colonne de contenu
    let mut content_col = column::with_capacity(3)
        .spacing(spacing.space_xxs)
        .push(
            row::with_capacity(2)
                .spacing(spacing.space_s)
                .align_y(Alignment::Center)
                .push(
                    text::body(icon_char)
                        .width(20.0)
                        .center(),
                )
                .push(
                    text::body(toast.title)
                        .width(Length::Fill),
                ),
        );

    if let Some(desc) = toast.description {
        content_col = content_col.push(
            text::caption(desc)
                .width(Length::Fill),
        );
    }

    // Ligne d'action
    let mut action_row = row::with_capacity(2)
        .spacing(spacing.space_s)
        .align_y(Alignment::Center);

    if let Some(action) = toast.action {
        let action_btn = mouse_area(
            container(
                text::caption(action.label)
                    .center(),
            )
            .padding(Padding::from([spacing.space_xxs, spacing.space_s])),
        )
        .on_press(action.on_press);
        action_row = action_row.push(action_btn);
    }

    action_row = action_row.push(space::horizontal().width(Length::Fill));

    if toast.dismissible {
        if let Some(dismiss) = on_dismiss {
            let close_btn = mouse_area(
                container(
                    text::body("✕").center(),
                )
                .width(24.0)
                .height(24.0)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            )
            .on_press(dismiss);
            action_row = action_row.push(close_btn);
        }
    }

    content_col = content_col.push(action_row);

    let toast_elem: Element<'static, Message> = container(content_col)
        .class(toast_class)
        .padding(Padding::from([
            spacing.space_s,
            spacing.space_m,
        ]))
        .width(Length::Fixed(360.0))
        .into();

    // Animation d'entrée (slide + lift)
    anim::animated(toast_elem)
        .preset(anim::AnimPreset::Lift {
            hover_scale: 1.0,
            press_scale: 0.98,
            hover_lift: -1.0,
        })
        .into()
}

// ═════════════════════════════════════════════════════════════════════════════
// TOAST VIEW — Liste de toasts empilés
// ═════════════════════════════════════════════════════════════════════════════

/// Vue de toasts empilés.
pub fn toast_view<'a, Message: Clone + 'static>(
    toasts: &'a [ToastItem<'static, Message>],
    position: ToastPosition,
    on_dismiss: impl Fn(ToastId) -> Message + 'static,
) -> Element<'static, Message> {
    let spacing = crate::theme::spacing();

    let mut col = column::with_capacity(toasts.len())
        .spacing(spacing.space_s);

    for toast in toasts {
        let dismiss_msg = on_dismiss(toast.id);
        col = col.push(toast_content(toast.clone(), Some(dismiss_msg)));
    }

    let aligned = match position {
        ToastPosition::TopRight | ToastPosition::BottomRight => {
            container(col.align_x(Alignment::End))
        }
        ToastPosition::TopLeft | ToastPosition::BottomLeft => {
            container(col)
        }
        ToastPosition::TopCenter | ToastPosition::BottomCenter => {
            container(col).center_x(Length::Fill)
        }
    };

    let padding = match position {
        ToastPosition::TopRight => Padding::from([20, 20, 0, 0]),
        ToastPosition::TopLeft => Padding::from([20, 0, 0, 20]),
        ToastPosition::TopCenter => Padding::from([20, 0, 0, 0]),
        ToastPosition::BottomRight => Padding::from([0, 20, 20, 0]),
        ToastPosition::BottomLeft => Padding::from([0, 0, 20, 20]),
        ToastPosition::BottomCenter => Padding::from([0, 0, 20, 0]),
    };

    aligned.padding(padding).into()
}

// ═════════════════════════════════════════════════════════════════════════════
// TOAST MANAGER — Gestionnaire de toasts
// ═════════════════════════════════════════════════════════════════════════════

/// Gestionnaire de toasts avec auto-dismiss.
pub struct ToastManager<Message> {
    toasts: Vec<ToastEntry<Message>>,
    max_visible: usize,
    position: ToastPosition,
}

struct ToastEntry<Message> {
    toast: ToastItem<'static, Message>,
    dismissed: bool,
}

impl<Message: Clone + 'static> Default for ToastManager<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message: Clone + 'static> ToastManager<Message> {
    /// Crée un nouveau gestionnaire.
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            max_visible: 5,
            position: ToastPosition::BottomRight,
        }
    }

    /// Nombre max de toasts visibles.
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    /// Position des toasts.
    pub fn position(mut self, pos: ToastPosition) -> Self {
        self.position = pos;
        self
    }

    /// Ajoute un toast.
    pub fn add(&mut self, toast: ToastItem<'static, Message>) {
        self.toasts.push(ToastEntry {
            toast,
            dismissed: false,
        });
        // Limiter le nombre de toasts
        if self.toasts.len() > self.max_visible {
            self.toasts.remove(0);
        }
    }

    /// Supprime un toast par son ID.
    pub fn dismiss(&mut self, id: ToastId) {
        self.toasts.retain(|e| e.toast.id != id);
    }

    /// Supprime tous les toasts.
    pub fn clear(&mut self) {
        self.toasts.clear();
    }

    /// Retourne les toasts actifs.
    pub fn active_toasts(&self) -> Vec<&ToastItem<'static, Message>> {
        self.toasts
            .iter()
            .filter(|e| !e.dismissed)
            .map(|e| &e.toast)
            .collect()
    }

    /// Génère la vue des toasts.
    pub fn view(&self, on_dismiss: impl Fn(ToastId) -> Message + 'static) -> Element<'static, Message> {
        let active: Vec<ToastItem<'static, Message>> = self
            .toasts
            .iter()
            .filter(|e| !e.dismissed)
            .map(|e| ToastItem {
                id: e.toast.id,
                variant: e.toast.variant,
                title: e.toast.title.clone(),
                description: e.toast.description.clone(),
                action: None, // Simplified for view
                dismissible: e.toast.dismissible,
            })
            .collect();

        if active.is_empty() {
            return space::horizontal().into();
        }

        toast_view(&active, self.position, on_dismiss)
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS — Création rapide de toasts
// ═════════════════════════════════════════════════════════════════════════════

/// Crée un toast d'information rapide.
pub fn toast_info<'a, Message: Clone + 'static>(
    id: usize,
    title: impl Into<Cow<'a, str>>,
    description: impl Into<Cow<'a, str>>,
) -> ToastItem<'a, Message> {
    ToastItem::new(id, ToastVariant::Info, title).description(description)
}

/// Crée un toast de succès rapide.
pub fn toast_success<'a, Message: Clone + 'static>(
    id: usize,
    title: impl Into<Cow<'a, str>>,
    description: impl Into<Cow<'a, str>>,
) -> ToastItem<'a, Message> {
    ToastItem::new(id, ToastVariant::Success, title).description(description)
}

/// Crée un toast d'avertissement rapide.
pub fn toast_warning<'a, Message: Clone + 'static>(
    id: usize,
    title: impl Into<Cow<'a, str>>,
    description: impl Into<Cow<'a, str>>,
) -> ToastItem<'a, Message> {
    ToastItem::new(id, ToastVariant::Warning, title).description(description)
}

/// Crée un toast d'erreur rapide.
pub fn toast_error<'a, Message: Clone + 'static>(
    id: usize,
    title: impl Into<Cow<'a, str>>,
    description: impl Into<Cow<'a, str>>,
) -> ToastItem<'a, Message> {
    ToastItem::new(id, ToastVariant::Error, title).description(description)
}
