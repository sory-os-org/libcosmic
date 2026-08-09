// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Styles SoryOS — Design System "Deep Navy Glass".
//!
//! Chaque fonction retourne un style réutilisable pour un widget COSMIC.
//! Utilise les tokens centralisés de `crate::palette::SORY`.
//!
//! Design tokens : gradients, glow effects, transparence, bordures lumineuses.

use crate::iced::gradient::Linear;
use crate::iced::{Background, Border, Color, Shadow, Vector};
use crate::palette;
use crate::palette::SORY;
use crate::theme;
use crate::widget::container;

// ═════════════════════════════════════════════════════════════════════════════
// HELPERS INTERNES
// ═════════════════════════════════════════════════════════════════════════════

fn glow_shadow(color: Color, blur: f32) -> Shadow {
    Shadow {
        color,
        offset: Vector::default(),
        blur_radius: blur,
    }
}

fn drop_shadow(color: Color, offset_y: f32, blur: f32) -> Shadow {
    Shadow {
        color,
        offset: Vector::new(0.0, offset_y),
        blur_radius: blur,
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// FONDS GLOBAUX
// ═════════════════════════════════════════════════════════════════════════════

/// Fond d'arrière-plan global le plus profond.
#[must_use]
pub fn bg_deep() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_DEEP)),
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

/// Fond de zone de travail principal.
#[must_use]
pub fn background() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_BASE)),
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

/// Fond de panneau avec gradient ambient glow bleu.
#[must_use]
pub fn bg_with_glow() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Gradient(
            SORY.gradient_ambient_glow().into(),
        )),
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// SIDEBAR
// ═════════════════════════════════════════════════════════════════════════════

/// Fond de sidebar avec gradient subtil.
#[must_use]
pub fn sidebar() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Gradient(SORY.gradient_sidebar().into())),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: 0.0.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Item de navigation sidebar — état inactif.
#[must_use]
pub fn sidebar_item() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_SECONDARY),
        text_color: Some(SORY.TEXT_SECONDARY),
        background: None,
        border: Border {
            radius: SORY.RADIUS_LIST_ITEM.into(),
            ..Default::default()
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Item de navigation sidebar — état actif (sélectionné).
#[must_use]
pub fn sidebar_item_active() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.ACCENT_BRIGHT),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.OVERLAY_SELECTED)),
        border: Border {
            color: SORY.BORDER_ACTIVE,
            width: 1.0,
            radius: SORY.RADIUS_LIST_ITEM.into(),
        },
        shadow: glow_shadow(SORY.GLOW_SUBTLE, 8.0),
        snap: true,
    })
}

/// Bande accent verticale dans la sidebar (indicateur actif).
#[must_use]
pub fn sidebar_accent_bar() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        background: Some(Background::Gradient(SORY.gradient_button().into())),
        border: Border {
            radius: [2.0, 2.0, 2.0, 2.0].into(),
            ..Default::default()
        },
        shadow: glow_shadow(SORY.GLOW_MEDIUM, 6.0),
        ..Default::default()
    })
}

/// Item de navigation sidebar — état parent actif (un sub-item est actif).
#[must_use]
pub fn sidebar_item_parent_active() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.ACCENT_LIGHT),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(with_alpha(SORY.ACCENT, 0.05))),
        border: Border {
            radius: SORY.RADIUS_LIST_ITEM.into(),
            ..Default::default()
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Footer de sidebar (branding SoryOS).
#[must_use]
pub fn sidebar_footer() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(with_alpha(SORY.BG_SURFACE, 0.5))),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// HEADER / BARRE DE RECHERCHE
// ═════════════════════════════════════════════════════════════════════════════

/// Barre d'en-tête supérieure.
#[must_use]
pub fn header_bar() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Gradient(SORY.gradient_header().into())),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 0.0,
            radius: [SORY.RADIUS_DIALOG, SORY.RADIUS_DIALOG, 0.0, 0.0].into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Conteneur barre de recherche.
#[must_use]
pub fn search_bar() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_MUTED),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.OVERLAY_HOVER)),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: SORY.RADIUS_PILL.into(),
        },
        shadow: drop_shadow(SORY.SHADOW, 1.0, 4.0),
        snap: true,
    })
}

/// Conteneur breadcrumb.
#[must_use]
pub fn breadcrumb() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_SECONDARY),
        text_color: Some(SORY.TEXT_SECONDARY),
        background: None,
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

/// Bouton de vue (grid/list/details) — état inactif.
#[must_use]
pub fn view_button() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_SECONDARY),
        text_color: Some(SORY.TEXT_SECONDARY),
        background: Some(Background::Color(SORY.OVERLAY_HOVER)),
        border: Border {
            radius: SORY.RADIUS_BUTTON.into(),
            ..Default::default()
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Bouton de vue — état actif.
#[must_use]
pub fn view_button_active() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_ON_ACCENT),
        text_color: Some(SORY.TEXT_ON_ACCENT),
        background: Some(Background::Gradient(SORY.gradient_button().into())),
        border: Border {
            radius: SORY.RADIUS_BUTTON.into(),
            ..Default::default()
        },
        shadow: glow_shadow(SORY.GLOW_SUBTLE, 6.0),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// CARTES (FOLDERS, FICHIERS)
// ═════════════════════════════════════════════════════════════════════════════

/// Carte dossier — état normal.
#[must_use]
pub fn folder_card() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.OVERLAY_HOVER)),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: drop_shadow(SORY.SHADOW, 2.0, 8.0),
        snap: true,
    })
}

/// Carte dossier — état survolé.
#[must_use]
pub fn folder_card_hover() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.OVERLAY_HOVER)),
        border: Border {
            color: SORY.BORDER_DEFAULT,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: glow_shadow(SORY.GLOW_SUBTLE, 12.0),
        snap: true,
    })
}

/// Carte dossier — état sélectionné (glow bleu actif).
#[must_use]
pub fn folder_card_selected() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.ACCENT_BRIGHT),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.OVERLAY_SELECTED)),
        border: Border {
            color: SORY.BORDER_GLOW,
            width: 1.5,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: glow_shadow(SORY.GLOW_STRONG, 20.0),
        snap: true,
    })
}

/// Tuile générique.
#[must_use]
pub fn tile() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.OVERLAY_HOVER)),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: drop_shadow(SORY.SHADOW, 1.0, 4.0),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// LISTE DE FICHIERS
// ═════════════════════════════════════════════════════════════════════════════

/// Style de section pour les listes SoryOS (fond transparent, bordure subtile).
#[must_use]
pub fn section() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_CONTENT)),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: SORY.RADIUS_SECTION.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Ligne de fichier — état normal.
#[must_use]
pub fn file_row() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: None,
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 0.0,
            radius: SORY.RADIUS_LIST_ITEM.into(),
            ..Default::default()
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Ligne de fichier — état survolé.
#[must_use]
pub fn file_row_hover() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.OVERLAY_HOVER)),
        border: Border {
            radius: SORY.RADIUS_LIST_ITEM.into(),
            ..Default::default()
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Ligne de fichier — état sélectionné.
#[must_use]
pub fn file_row_selected() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.OVERLAY_SELECTED)),
        border: Border {
            color: SORY.BORDER_ACTIVE,
            width: 1.0,
            radius: SORY.RADIUS_LIST_ITEM.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// DÉTAILS PANEL (panneau droit)
// ═════════════════════════════════════════════════════════════════════════════

/// Fond du panneau de détails.
#[must_use]
pub fn details_panel() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_SURFACE)),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: [SORY.RADIUS_DIALOG, 0.0, 0.0, SORY.RADIUS_DIALOG].into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Icône de dossier dans les détails (avec glow bleu).
#[must_use]
pub fn details_folder_icon() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.ACCENT_BRIGHT),
        text_color: Some(SORY.ACCENT_BRIGHT),
        background: Some(Background::Color(with_alpha(SORY.ACCENT, 0.10))),
        border: Border {
            color: SORY.BORDER_ACTIVE,
            width: 1.5,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: glow_shadow(SORY.GLOW_MEDIUM, 16.0),
        snap: true,
    })
}

/// Ligne de métadonnée dans les détails (icône + label + valeur).
#[must_use]
pub fn details_meta_row() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_SECONDARY),
        text_color: Some(SORY.TEXT_SECONDARY),
        background: None,
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// BOUTONS
// ═════════════════════════════════════════════════════════════════════════════

/// Bouton principal avec gradient bleu.
#[must_use]
pub fn button_primary() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_ON_ACCENT),
        text_color: Some(SORY.TEXT_ON_ACCENT),
        background: Some(Background::Gradient(SORY.gradient_button().into())),
        border: Border {
            radius: SORY.RADIUS_BUTTON.into(),
            ..Default::default()
        },
        shadow: glow_shadow(SORY.GLOW_SUBTLE, 8.0),
        snap: true,
    })
}

/// Bouton principal — état survolé.
#[must_use]
pub fn button_primary_hover() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_ON_ACCENT),
        text_color: Some(SORY.TEXT_ON_ACCENT),
        background: Some(Background::Gradient(SORY.gradient_button_hover().into())),
        border: Border {
            radius: SORY.RADIUS_BUTTON.into(),
            ..Default::default()
        },
        shadow: glow_shadow(SORY.GLOW_MEDIUM, 12.0),
        snap: true,
    })
}

/// Bouton secondaire (outline).
#[must_use]
pub fn button_secondary() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.OVERLAY_HOVER)),
        border: Border {
            color: SORY.BORDER_DEFAULT,
            width: 1.0,
            radius: SORY.RADIUS_BUTTON.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Bouton icône (circle).
#[must_use]
pub fn button_icon() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_SECONDARY),
        text_color: Some(SORY.TEXT_SECONDARY),
        background: Some(Background::Color(SORY.OVERLAY_HOVER)),
        border: Border {
            radius: SORY.RADIUS_PILL.into(),
            ..Default::default()
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Bouton More (⋯).
#[must_use]
pub fn button_more() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_MUTED),
        text_color: Some(SORY.TEXT_MUTED),
        background: None,
        border: Border {
            radius: SORY.RADIUS_PILL.into(),
            ..Default::default()
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// DIALOGUES
// ═════════════════════════════════════════════════════════════════════════════

/// Cadre extérieur d'un dialogue.
#[must_use]
pub fn dialog_frame() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_MODAL)),
        border: Border {
            color: SORY.BORDER_DIALOG,
            width: 1.5,
            radius: SORY.RADIUS_DIALOG.into(),
        },
        shadow: drop_shadow(SORY.SHADOW, 8.0, 32.0),
        snap: true,
    })
}

/// Panneau intérieur d'un dialogue.
#[must_use]
pub fn dialog_panel() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_ELEVATED)),
        border: Border {
            color: SORY.BORDER_DEFAULT,
            width: 1.0,
            radius: (SORY.RADIUS_DIALOG - 2.0).max(0.0).into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Barre de boutons en pied de dialogue.
#[must_use]
pub fn dialog_button_bar() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_MODAL)),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: [0.0, 0.0, SORY.RADIUS_DIALOG, SORY.RADIUS_DIALOG].into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// CONTEXT DRAWER
// ═════════════════════════════════════════════════════════════════════════════

/// Fond du panneau latéral (context drawer).
#[must_use]
pub fn context_drawer() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_CONTEXT)),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: [SORY.RADIUS_DIALOG, 0.0, 0.0, SORY.RADIUS_DIALOG].into(),
        },
        shadow: drop_shadow(SORY.SHADOW, 0.0, 16.0),
        snap: true,
    })
}

/// Contenu du context drawer.
#[must_use]
pub fn context_content() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(with_alpha(SORY.BG_ELEVATED, 0.6))),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// ÉLÉMENTS D'ICÔNE ET BADGES
// ═════════════════════════════════════════════════════════════════════════════

/// Puits d'icône coloré paramétrable.
#[must_use]
pub fn tile_icon(color: Color) -> theme::Container<'static> {
    theme::Container::custom(move |_| container::Style {
        icon_color: Some(color),
        text_color: Some(color),
        background: Some(Background::Color(with_alpha(color, 0.12))),
        border: Border {
            radius: SORY.RADIUS_ICON.into(),
            ..Default::default()
        },
        shadow: glow_shadow(with_alpha(color, 0.15), 8.0),
        ..Default::default()
    })
}

/// Puits d'icône bleu.
#[must_use]
pub fn tile_icon_blue() -> theme::Container<'static> {
    tile_icon(SORY.ACCENT_BRIGHT)
}

/// Puits d'icône violet.
#[must_use]
pub fn tile_icon_purple() -> theme::Container<'static> {
    tile_icon(SORY.ACCENT_PURPLE)
}

/// Puits d'icône vert.
#[must_use]
pub fn tile_icon_green() -> theme::Container<'static> {
    tile_icon(SORY.ACCENT_GREEN)
}

/// Puits d'icône orange.
#[must_use]
pub fn tile_icon_orange() -> theme::Container<'static> {
    tile_icon(SORY.ACCENT_ORANGE)
}

/// Badge / chip d'information.
#[must_use]
pub fn chip() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.ACCENT_BRIGHT),
        text_color: Some(SORY.ACCENT_BRIGHT),
        background: Some(Background::Color(with_alpha(SORY.ACCENT, 0.10))),
        border: Border {
            color: SORY.BORDER_ACTIVE,
            width: 1.0,
            radius: SORY.RADIUS_PILL.into(),
        },
        ..Default::default()
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// STATUS BAR
// ═════════════════════════════════════════════════════════════════════════════

/// Barre de statut en bas de la fenêtre.
#[must_use]
pub fn status_bar() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_SECONDARY),
        text_color: Some(SORY.TEXT_SECONDARY),
        background: Some(Background::Color(with_alpha(SORY.BG_SURFACE, 0.6))),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: [0.0, 0.0, SORY.RADIUS_DIALOG, SORY.RADIUS_DIALOG].into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// PROGRESSION / INDICATEURS
// ═════════════════════════════════════════════════════════════════════════════

/// Style local pour les barres de progression (remplace `crate::theme::progress_bar` inexistant).
pub struct ProgressBarStyle {
    pub background: Background,
    pub bar: Background,
    pub border_radius: crate::iced::border::Radius,
}

/// Remplissage de barre de progression SoryOS (gradient bleu).
#[must_use]
pub fn progress_fill() -> ProgressBarStyle {
    ProgressBarStyle {
        background: Background::Color(with_alpha(SORY.ACCENT, 0.15)),
        bar: Background::Gradient(SORY.gradient_button().into()),
        border_radius: SORY.RADIUS_PILL.into(),
    }
}

/// Piste de progression.
#[must_use]
pub fn progress_track() -> ProgressBarStyle {
    ProgressBarStyle {
        background: Background::Color(with_alpha(Color::WHITE, 0.05)),
        bar: Background::Gradient(SORY.gradient_button().into()),
        border_radius: SORY.RADIUS_PILL.into(),
    }
}

/// Indicateur drag-and-drop.
#[must_use]
pub fn dnd_indicator() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        background: Some(Background::Color(SORY.OVERLAY_SELECTED)),
        border: Border {
            color: SORY.BORDER_GLOW,
            width: 2.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: glow_shadow(SORY.GLOW_MEDIUM, 12.0),
        ..Default::default()
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// DÉCORATIONS DE FOND
// ═════════════════════════════════════════════════════════════════════════════

/// Éclat bleu décoratif (ambient glow).
#[must_use]
pub fn glow_blue() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        background: Some(Background::Color(with_alpha(SORY.ACCENT, 0.12))),
        shadow: Shadow {
            color: with_alpha(SORY.ACCENT, 0.35),
            offset: Vector::default(),
            blur_radius: 100.0,
        },
        border: Border {
            radius: 500.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
}

/// Éclat violet décoratif.
#[must_use]
pub fn glow_violet() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        background: Some(Background::Color(with_alpha(SORY.ACCENT_PURPLE, 0.10))),
        shadow: Shadow {
            color: with_alpha(SORY.ACCENT_PURPLE, 0.30),
            offset: Vector::default(),
            blur_radius: 120.0,
        },
        border: Border {
            radius: 500.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// CONTRÔLES (TOGGLE, CHECKBOX)
// ═════════════════════════════════════════════════════════════════════════════

/// Piste du toggle SoryOS — état ON (gradient bleu).
#[must_use]
pub fn toggle_track_on() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        background: Some(Background::Gradient(SORY.gradient_button().into())),
        border: Border {
            radius: SORY.RADIUS_PILL.into(),
            ..Default::default()
        },
        shadow: glow_shadow(SORY.GLOW_SUBTLE, 6.0),
        ..Default::default()
    })
}

/// Piste du toggle — état OFF.
#[must_use]
pub fn toggle_track_off() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        background: Some(Background::Color(with_alpha(Color::WHITE, 0.10))),
        border: Border {
            radius: SORY.RADIUS_PILL.into(),
            ..Default::default()
        },
        ..Default::default()
    })
}

/// Knob du toggle.
#[must_use]
pub fn toggle_knob() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            radius: SORY.RADIUS_PILL.into(),
            ..Default::default()
        },
        shadow: drop_shadow(SORY.SHADOW, 1.0, 3.0),
        ..Default::default()
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// ÉDITORS / DISPLAY
// ═════════════════════════════════════════════════════════════════════════════

/// Panneau d'édition.
#[must_use]
pub fn edit_panel() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_ELEVATED)),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Cadre aperçu écran.
#[must_use]
pub fn display_frame() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        border: Border {
            color: SORY.BORDER_DEFAULT,
            width: 2.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
}

/// Fond écran display.
#[must_use]
pub fn display_screen() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        background: Some(Background::Color(SORY.BG_DEEP)),
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
}

/// Aperçu fond d'écran.
#[must_use]
pub fn wallpaper_preview() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: drop_shadow(SORY.SHADOW, 2.0, 8.0),
        ..Default::default()
    })
}

/// Popup déroulant.
#[must_use]
pub fn dropdown() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_ELEVATED)),
        border: Border {
            color: SORY.BORDER_DEFAULT,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: drop_shadow(SORY.SHADOW, 4.0, 16.0),
        snap: true,
    })
}

/// Bannière notification.
#[must_use]
pub fn notification() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_ELEVATED)),
        border: Border {
            color: SORY.BORDER_DEFAULT,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: drop_shadow(SORY.SHADOW, 4.0, 16.0),
        snap: true,
    })
}

/// Carte d'information.
#[must_use]
pub fn info_card() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(with_alpha(SORY.ACCENT, 0.06))),
        border: Border {
            color: SORY.BORDER_DEFAULT,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// NAVIGATION — SÉPARATEURS ET ONGLETS
// ═════════════════════════════════════════════════════════════════════════════

/// Séparateur horizontal dans la sidebar.
#[must_use]
pub fn sidebar_separator() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BORDER_SUBTLE)),
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

/// Barre d'onglets SoryOS (fond avec bordure inférieure).
#[must_use]
pub fn tab_bar() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_BASE)),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: [SORY.RADIUS_CARD, SORY.RADIUS_CARD, 0.0, 0.0].into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Sous-ligne inactive des onglets (trait gris subtil).
#[must_use]
pub fn tab_underline_inactive() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_SECONDARY),
        text_color: Some(SORY.TEXT_SECONDARY),
        background: Some(Background::Color(with_alpha(Color::WHITE, 0.06))),
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// TOAST / NOTIFICATION
// ═════════════════════════════════════════════════════════════════════════════

/// Fond d'un toast/notification.
#[must_use]
pub fn toast() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_ELEVATED)),
        border: Border {
            color: SORY.BORDER_DEFAULT,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: drop_shadow(SORY.SHADOW, 4.0, 16.0),
        snap: true,
    })
}

/// Toast de succès.
#[must_use]
pub fn toast_success() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.ACCENT_GREEN),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_ELEVATED)),
        border: Border {
            color: with_alpha(SORY.ACCENT_GREEN, 0.4),
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: drop_shadow(with_alpha(SORY.ACCENT_GREEN, 0.2), 4.0, 16.0),
        snap: true,
    })
}

/// Toast d'erreur.
#[must_use]
pub fn toast_error() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.ACCENT_RED),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_ELEVATED)),
        border: Border {
            color: with_alpha(SORY.ACCENT_RED, 0.4),
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: drop_shadow(with_alpha(SORY.ACCENT_RED, 0.2), 4.0, 16.0),
        snap: true,
    })
}

/// Toast d'avertissement.
#[must_use]
pub fn toast_warning() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.ACCENT_ORANGE),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_ELEVATED)),
        border: Border {
            color: with_alpha(SORY.ACCENT_ORANGE, 0.4),
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: drop_shadow(with_alpha(SORY.ACCENT_ORANGE, 0.2), 4.0, 16.0),
        snap: true,
    })
}

/// Toast d'information.
#[must_use]
pub fn toast_info() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.ACCENT_BRIGHT),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_ELEVATED)),
        border: Border {
            color: with_alpha(SORY.ACCENT_BRIGHT, 0.4),
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: drop_shadow(with_alpha(SORY.ACCENT_BRIGHT, 0.2), 4.0, 16.0),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// POPOVER
// ═════════════════════════════════════════════════════════════════════════════

/// Fond d'un popover.
#[must_use]
pub fn popover() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_ELEVATED)),
        border: Border {
            color: SORY.BORDER_DEFAULT,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: drop_shadow(SORY.SHADOW, 4.0, 16.0),
        snap: true,
    })
}

/// Contenu de popover.
#[must_use]
pub fn popover_content() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.BG_ELEVATED)),
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// EMPTY STATE / LOADING
// ═════════════════════════════════════════════════════════════════════════════

/// Fond d'état vide (aucun contenu).
#[must_use]
pub fn empty_state() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_MUTED),
        text_color: Some(SORY.TEXT_MUTED),
        background: None,
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

/// Conteneur de chargement.
#[must_use]
pub fn loading_state() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.ACCENT_BRIGHT),
        text_color: Some(SORY.TEXT_SECONDARY),
        background: None,
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

/// Carte sélectionnée avec glow bleu (pour sélection multiple).
#[must_use]
pub fn card_selectable() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.TEXT_PRIMARY),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.OVERLAY_HOVER)),
        border: Border {
            color: SORY.BORDER_SUBTLE,
            width: 1.0,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Carte sélectionnée avec glow bleu.
#[must_use]
pub fn card_selectable_active() -> theme::Container<'static> {
    theme::Container::custom(|_| container::Style {
        icon_color: Some(SORY.ACCENT_BRIGHT),
        text_color: Some(SORY.TEXT_PRIMARY),
        background: Some(Background::Color(SORY.OVERLAY_SELECTED)),
        border: Border {
            color: SORY.BORDER_GLOW,
            width: 1.5,
            radius: SORY.RADIUS_CARD.into(),
        },
        shadow: glow_shadow(SORY.GLOW_MEDIUM, 12.0),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// HELPERS
// ═════════════════════════════════════════════════════════════════════════════

fn with_alpha(color: Color, a: f32) -> Color {
    Color { a, ..color }
}
