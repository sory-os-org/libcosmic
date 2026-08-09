// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Palette de couleurs SoryOS — système de design tokens.
//!
//! Centralise toutes les couleurs, gradients, et constantes visuelles
//! du design system SoryOS. Basé sur le design "Deep Navy Glass" :
//! fond sombre profond, accents bleus lumineux, effets glow et transparence.
//!
//! Utilisation : `use cosmic::palette;` puis `palette::SORY.*`

use crate::iced::gradient::Linear;
use crate::iced::{Color, Radians};

// ── Utilitaires ──────────────────────────────────────────────────────────────

/// Convertit un code hexadécimal en `Color` RGB (alpha = 1.0).
#[must_use]
#[inline]
pub const fn rgb(hex: u32) -> Color {
    Color::from_rgb(
        ((hex >> 16) & 0xff) as f32 / 255.0,
        ((hex >> 8) & 0xff) as f32 / 255.0,
        (hex & 0xff) as f32 / 255.0,
    )
}

/// Crée une `Color` avec un alpha spécifique.
#[must_use]
#[inline]
pub const fn with_alpha(color: Color, a: f32) -> Color {
    Color { a, ..color }
}

/// Crée un gradient linéaire de haut en bas.
#[must_use]
pub fn gradient_v(color_from: Color, color_to: Color) -> Linear {
    Linear::new(Radians(1.5708)) // 90° = vers le bas
        .add_stop(0.0, color_from)
        .add_stop(1.0, color_to)
}

/// Crée un gradient linéaire diagonal (haut-gauche → bas-droite).
#[must_use]
pub fn gradient_diagonal(color_from: Color, color_to: Color) -> Linear {
    Linear::new(Radians(0.7854)) // 45°
        .add_stop(0.0, color_from)
        .add_stop(1.0, color_to)
}

// ═════════════════════════════════════════════════════════════════════════════
// SORY DESIGN TOKENS
// ═════════════════════════════════════════════════════════════════════════════

/// Tous les tokens de design SoryOS — accéder via `palette::SORY`.
pub static SORY: SoryTokens = SoryTokens {
    // Backgrounds
    BG_DEEP: rgb(0x030712),
    BG_BASE: rgb(0x05091b),
    BG_SURFACE: rgb(0x0a1229),
    BG_ELEVATED: rgb(0x0e1a35),
    BG_MODAL: rgb(0x060c20),
    BG_CONTEXT: rgb(0x0d1530),
    BG_CONTENT: rgb(0x080e24),
    // Text
    TEXT_PRIMARY: rgb(0xe9eeff),
    TEXT_SECONDARY: rgb(0x9aa5cc),
    TEXT_MUTED: rgb(0x5a6690),
    TEXT_ON_ACCENT: Color::WHITE,
    // Accent colors
    ACCENT: rgb(0x2b6de8),
    ACCENT_BRIGHT: rgb(0x4a8aff),
    ACCENT_LIGHT: rgb(0x6ba3ff),
    ACCENT_DARK: rgb(0x1a3d8f),
    ACCENT_PURPLE: rgb(0x7c5bf5),
    ACCENT_GREEN: rgb(0x22c55e),
    ACCENT_ORANGE: rgb(0xf59e0b),
    ACCENT_RED: rgb(0xef4444),
    ACCENT_CYAN: rgb(0x06b6d4),
    ACCENT_PINK: rgb(0xec4899),
    // Borders
    BORDER_DEFAULT: with_alpha(rgb(0x2b6de8), 0.15),
    BORDER_ACTIVE: with_alpha(rgb(0x4a8aff), 0.5),
    BORDER_SUBTLE: with_alpha(Color::WHITE, 0.06),
    BORDER_DIALOG: with_alpha(rgb(0x2b6de8), 0.4),
    BORDER_GLOW: with_alpha(rgb(0x4a8aff), 0.6),
    // Shadows & Glows
    GLOW_SUBTLE: with_alpha(rgb(0x2b6de8), 0.15),
    GLOW_MEDIUM: with_alpha(rgb(0x4a8aff), 0.3),
    GLOW_STRONG: with_alpha(rgb(0x2b6de8), 0.5),
    SHADOW: with_alpha(Color::BLACK, 0.3),
    // Overlay
    OVERLAY_HOVER: with_alpha(Color::WHITE, 0.05),
    OVERLAY_PRESSED: with_alpha(Color::WHITE, 0.08),
    OVERLAY_SELECTED: with_alpha(rgb(0x2b6de8), 0.12),
    // Spacing & Radius
    RADIUS_PILL: 999.0,
    RADIUS_CARD: 12.0,
    RADIUS_DIALOG: 16.0,
    RADIUS_BUTTON: 10.0,
    RADIUS_SECTION: 8.0,
    RADIUS_ICON: 14.0,
    RADIUS_LIST_ITEM: 6.0,
};

#[allow(non_snake_case)]
pub struct SoryTokens {
    // ── Backgrounds ──────────────────────────────────────────────────────
    /// Fond principal très sombre (derrière tout).
    pub BG_DEEP: Color,
    /// Fond principal sombre (zone de travail).
    pub BG_BASE: Color,
    /// Fond de sidebar / panneau latéral.
    pub BG_SURFACE: Color,
    /// Fond de carte / section légèrement plus clair.
    pub BG_ELEVATED: Color,
    /// Fond de dialogue modale.
    pub BG_MODAL: Color,
    /// Fond de context drawer.
    pub BG_CONTEXT: Color,
    /// Fond de contenu principal.
    pub BG_CONTENT: Color,

    // ── Text ─────────────────────────────────────────────────────────────
    /// Texte principal (titres, labels).
    pub TEXT_PRIMARY: Color,
    /// Texte secondaire (descriptions, métadonnées).
    pub TEXT_SECONDARY: Color,
    /// Texte atténué (hints, placeholders).
    pub TEXT_MUTED: Color,
    /// Texte sur accent coloré.
    pub TEXT_ON_ACCENT: Color,

    // ── Accent colors ────────────────────────────────────────────────────
    /// Bleu accent principal (navigation active, boutons, liens).
    pub ACCENT: Color,
    /// Bleu accent bright (hover, focus ring).
    pub ACCENT_BRIGHT: Color,
    /// Bleu accent léger (dividers, bordures subtiles).
    pub ACCENT_LIGHT: Color,
    /// Bleu accent très sombre (backgrounds d'accent).
    pub ACCENT_DARK: Color,
    /// Violet accent (tags, éléments décoratifs).
    pub ACCENT_PURPLE: Color,
    /// Vert accent (succès, fichiers, toggle on).
    pub ACCENT_GREEN: Color,
    /// Orange accent (warning, archives).
    pub ACCENT_ORANGE: Color,
    /// Rouge accent (destructif, suppression).
    pub ACCENT_RED: Color,
    /// Cyan accent (liens, éléments interactifs).
    pub ACCENT_CYAN: Color,
    /// Rose accent (favoris, étoiles).
    pub ACCENT_PINK: Color,

    // ── Borders ──────────────────────────────────────────────────────────
    /// Bordure principale (cartes, sections).
    pub BORDER_DEFAULT: Color,
    /// Bordure active (sélection, focus).
    pub BORDER_ACTIVE: Color,
    /// Bordure subtile (dividers).
    pub BORDER_SUBTLE: Color,
    /// Bordure dialogue.
    pub BORDER_DIALOG: Color,
    /// Bordure glow (effet lumineux).
    pub BORDER_GLOW: Color,

    // ── Shadows & Glows ──────────────────────────────────────────────────
    /// Glow bleu faible (cartes au survol).
    pub GLOW_SUBTLE: Color,
    /// Glow bleu moyen (cartes sélectionnées).
    pub GLOW_MEDIUM: Color,
    /// Glow bleu fort (éléments actifs).
    pub GLOW_STRONG: Color,
    /// Ombre noire portée.
    pub SHADOW: Color,

    // ── Overlay ──────────────────────────────────────────────────────────
    /// Overlay hover.
    pub OVERLAY_HOVER: Color,
    /// Overlay pressed.
    pub OVERLAY_PRESSED: Color,
    /// Overlay selected.
    pub OVERLAY_SELECTED: Color,

    // ── Spacing & Radius ─────────────────────────────────────────────────
    /// Rayon pour les boutons pills / toggles.
    pub RADIUS_PILL: f32,
    /// Rayon pour les cartes.
    pub RADIUS_CARD: f32,
    /// Rayon pour les dialogues.
    pub RADIUS_DIALOG: f32,
    /// Rayon pour les boutons.
    pub RADIUS_BUTTON: f32,
    /// Rayon pour les sections.
    pub RADIUS_SECTION: f32,
    /// Rayon pour les icônes.
    pub RADIUS_ICON: f32,
    /// Rayon pour les items de liste.
    pub RADIUS_LIST_ITEM: f32,
}

impl SoryTokens {
    /// Gradient bouton principal (bleu, de haut en bas).
    pub fn gradient_button(&self) -> Linear {
        gradient_v(rgb(0x3a7bff), rgb(0x1a5adf))
    }

    /// Gradient bouton hover.
    pub fn gradient_button_hover(&self) -> Linear {
        gradient_v(rgb(0x4a8aff), rgb(0x2a6aef))
    }

    /// Gradient sidebar (de haut en bas, très subtil).
    pub fn gradient_sidebar(&self) -> Linear {
        gradient_v(
            with_alpha(self.ACCENT, 0.03),
            with_alpha(self.ACCENT, 0.0),
        )
    }

    /// Gradient header (de gauche à droite).
    pub fn gradient_header(&self) -> Linear {
        gradient_v(
            with_alpha(self.ACCENT, 0.06),
            with_alpha(Color::WHITE, 0.02),
        )
    }

    /// Gradient ambient glow (effet lumineux en arrière-plan).
    pub fn gradient_ambient_glow(&self) -> Linear {
        Linear::new(Radians(0.0))
            .add_stop(0.0, with_alpha(self.ACCENT, 0.12))
            .add_stop(0.5, with_alpha(self.ACCENT, 0.04))
            .add_stop(1.0, with_alpha(self.ACCENT, 0.0))
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// COULEURS PAR TYPE DE FICHIER (pour les icônes)
// ═════════════════════════════════════════════════════════════════════════════

/// Couleurs associées aux types de fichiers.
pub struct FileColors;

impl FileColors {
    /// PDF (rouge).
    pub const PDF: Color = rgb(0xef4444);
    /// Document texte (bleu clair).
    pub const DOCUMENT: Color = rgb(0x3b82f6);
    /// Image (vert).
    pub const IMAGE: Color = rgb(0x22c55e);
    /// Audio (violet).
    pub const AUDIO: Color = rgb(0x8b5cf6);
    /// Vidéo (rose).
    pub const VIDEO: Color = rgb(0xec4899);
    /// Archive (orange).
    pub const ARCHIVE: Color = rgb(0xf59e0b);
    /// Script/code (cyan).
    pub const CODE: Color = rgb(0x06b6d4);
    /// Dossier (bleu).
    pub const FOLDER: Color = rgb(0x3b82f6);
    /// Inconnu (gris).
    pub const UNKNOWN: Color = rgb(0x6b7280);
}
