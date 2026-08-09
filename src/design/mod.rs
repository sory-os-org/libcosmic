//! Design System SoryOS — tokens centralisés pour l'ensemble de l'UI.
//!
//! Ce module définit tous les tokens de design utilisés par les composants :
//! couleurs, espacements, rayons, ombres, typographie, motion.
//!
//! Les tokens sont organisés par domaine et fournissent des valeurs
//! cohérentes pour le dark mode et le light mode.

pub mod color;
pub mod motion;
pub mod radius;
pub mod shadow;
pub mod spacing;
pub mod style_builder;
pub mod typography;

pub use color::{AccentPalette, Palette, PrimaryPalette, NeutralPalette, SemanticColor, StatusPalette, DARK, LIGHT};
pub use motion::{DURATIONS, TransitionType, SlideDirection};
pub use radius::{RADIUS, Radii};
pub use shadow::{SHADOWS_DARK, SHADOWS_LIGHT, Shadows, ShadowToken};
pub use spacing::{SPACING, Spacing};
pub use typography::{TYPOGRAPHY, Typography};

/// Design System complet — contient tous les tokens pour un mode donné.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DesignSystem {
    /// Palette de couleurs.
    pub color: Palette,
    /// Échelle d'espacements.
    pub spacing: Spacing,
    /// Rayons de bordure.
    pub radius: Radii,
    /// Ombres et lueurs.
    pub shadows: Shadows,
    /// Typographie.
    pub typography: Typography,
}

impl DesignSystem {
    /// Version dark mode — Deep Navy Glass.
    pub const DARK: Self = Self {
        color: DARK,
        spacing: SPACING,
        radius: RADIUS,
        shadows: SHADOWS_DARK,
        typography: TYPOGRAPHY,
    };

    /// Version light mode.
    pub const LIGHT: Self = Self {
        color: LIGHT,
        spacing: SPACING,
        radius: RADIUS,
        shadows: SHADOWS_LIGHT,
        typography: TYPOGRAPHY,
    };
}
