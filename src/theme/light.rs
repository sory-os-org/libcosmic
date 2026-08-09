// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Light Theme pour COSMIC — styles fond clair épuré.
//!
//! Ce module expose les styles light theme pour être utilisés par
//! sory-desktop et d'autres applications COSMIC qui préfèrent un thème clair.

use crate::iced::{Background, Border, Color, Shadow, Vector};
use crate::widget::container;

// ═════════════════════════════════════════════════════════════════════════════
// HELPERS INTERNES
// ═════════════════════════════════════════════════════════════════════════════

fn drop_shadow(color: Color, offset_y: f32, blur: f32) -> Shadow {
    Shadow {
        color,
        offset: Vector::new(0.0, offset_y),
        blur_radius: blur,
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// SIDEBAR
// ═════════════════════════════════════════════════════════════════════════════

/// Fond de sidebar light.
#[must_use]
pub fn sidebar() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.11, 0.11, 0.11)),
        text_color: Some(Color::from_rgb(0.11, 0.11, 0.11)),
        background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
        border: Border {
            color: Color::from_rgb(0.9, 0.9, 0.9),
            width: 1.0,
            radius: 0.0.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Item de navigation sidebar — état inactif.
#[must_use]
pub fn sidebar_item() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.42, 0.44, 0.5)),
        text_color: Some(Color::from_rgb(0.42, 0.44, 0.5)),
        background: None,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
            ..Default::default()
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Item de navigation sidebar — état actif.
#[must_use]
pub fn sidebar_item_active() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.23, 0.51, 0.96)),
        text_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        background: Some(Background::Color(Color::from_rgb(0.93, 0.95, 1.0))),
        border: Border {
            color: Color::from_rgb(0.23, 0.51, 0.96),
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Bande accent verticale dans la sidebar (indicateur actif).
#[must_use]
pub fn sidebar_accent_bar() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.23, 0.51, 0.96))),
        border: Border {
            radius: [0.0, 4.0, 4.0, 0.0].into(),
            ..Default::default()
        },
        shadow: Default::default(),
        ..Default::default()
    })
}

/// Footer de sidebar.
#[must_use]
pub fn sidebar_footer() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        text_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        background: Some(Background::Color(Color::from_rgb(0.95, 0.96, 0.98))),
        border: Border {
            color: Color::from_rgb(0.9, 0.9, 0.9),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// HEADER
// ═════════════════════════════════════════════════════════════════════════════

/// Barre d'en-tête light.
#[must_use]
pub fn header_bar() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        text_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            color: Color::from_rgb(0.95, 0.96, 0.98),
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: drop_shadow(Color::from_rgba(0, 0, 0, 0.05), 1.0, 3.0),
        snap: true,
    })
}

/// Status badge light.
#[must_use]
pub fn status_badge() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        text_color: Some(Color::from_rgb(0.42, 0.44, 0.5)),
        background: Some(Background::Color(Color::from_rgb(0.95, 0.96, 0.98))),
        border: Border {
            color: Color::from_rgb(0.9, 0.9, 0.9),
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// CARDS
// ═════════════════════════════════════════════════════════════════════════════

/// Carte light — état normal.
#[must_use]
pub fn card() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        text_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            color: Color::from_rgb(0.9, 0.9, 0.9),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: drop_shadow(Color::from_rgba(0, 0, 0, 0.05), 1.0, 3.0),
        snap: true,
    })
}

/// Carte light — état hover.
#[must_use]
pub fn card_hover() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        text_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        background: Some(Background::Color(Color::from_rgb(0.95, 0.96, 0.98))),
        border: Border {
            color: Color::from_rgb(0.82, 0.84, 0.86),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: drop_shadow(Color::from_rgba(0, 0, 0, 0.1), 2.0, 4.0),
        snap: true,
    })
}

/// Carte light — état sélectionné.
#[must_use]
pub fn card_selected() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.23, 0.51, 0.96)),
        text_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        background: Some(Background::Color(Color::from_rgb(0.93, 0.95, 1.0))),
        border: Border {
            color: Color::from_rgb(0.23, 0.51, 0.96),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: drop_shadow(Color::from_rgba(0, 0, 0, 0.1), 2.0, 4.0),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// INPUT AREA
// ═════════════════════════════════════════════════════════════════════════════

/// Zone de saisie light.
#[must_use]
pub fn input_area() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        text_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            color: Color::from_rgb(0.9, 0.9, 0.9),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: drop_shadow(Color::from_rgba(0, 0, 0, 0.05), 1.0, 2.0),
        snap: true,
    })
}

/// Zone de saisie light — état focus.
#[must_use]
pub fn input_area_focus() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        text_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            color: Color::from_rgb(0.23, 0.51, 0.96),
            width: 2.0,
            radius: 8.0.into(),
        },
        shadow: drop_shadow(Color::from_rgba(0, 0, 0, 0.1), 1.0, 3.0),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// BUTTONS
// ═════════════════════════════════════════════════════════════════════════════

/// Bouton principal light.
#[must_use]
pub fn button_primary() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::WHITE),
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(Color::from_rgb(0.23, 0.51, 0.96))),
        border: Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        shadow: drop_shadow(Color::from_rgba(0, 0, 0, 0.05), 1.0, 2.0),
        snap: true,
    })
}

/// Bouton principal light — état hover.
#[must_use]
pub fn button_primary_hover() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::WHITE),
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(Color::from_rgb(0.15, 0.39, 0.84))),
        border: Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        shadow: drop_shadow(Color::from_rgba(0, 0, 0, 0.1), 1.0, 3.0),
        snap: true,
    })
}

/// Bouton secondaire light.
#[must_use]
pub fn button_secondary() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        text_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        background: Some(Background::Color(Color::from_rgb(0.95, 0.96, 0.98))),
        border: Border {
            color: Color::from_rgb(0.9, 0.9, 0.9),
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// BACKGROUND
// ═════════════════════════════════════════════════════════════════════════════

/// Fond principal light.
#[must_use]
pub fn background() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        text_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        background: Some(Background::Color(Color::WHITE)),
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

/// Fond de contenu light.
#[must_use]
pub fn content_background() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        text_color: Some(Color::from_rgb(0.07, 0.09, 0.15)),
        background: Some(Background::Color(Color::WHITE)),
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}
