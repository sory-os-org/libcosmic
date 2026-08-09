//! Rayons de bordure du Design System.

use iced_core::border::Radius;

/// Tokens de rayon de bordure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radii {
    /// 0px — pas d'arrondi
    pub none: f32,
    /// 2px — très subtil
    pub xs: f32,
    /// 4px — subtil
    pub sm: f32,
    /// 6px — standard
    pub md: f32,
    /// 8px — modéré
    pub lg: f32,
    /// 10px — boutons
    pub button: f32,
    /// 12px — cartes
    pub card: f32,
    /// 14px — icônes
    pub icon: f32,
    /// 16px — dialogues
    pub dialog: f32,
    /// 24px — généreux
    pub xl: f32,
    /// 999px — pill / rounded full
    pub pill: f32,
}

/// Rayons par défaut.
pub const RADIUS: Radii = Radii {
    none: 0.0,
    xs: 2.0,
    sm: 4.0,
    md: 6.0,
    lg: 8.0,
    button: 10.0,
    card: 12.0,
    icon: 14.0,
    dialog: 16.0,
    xl: 24.0,
    pill: 999.0,
};

impl Radii {
    pub const fn as_slice(&self) -> [f32; 11] {
        [
            self.none, self.xs, self.sm, self.md, self.lg,
            self.button, self.card, self.icon, self.dialog,
            self.xl, self.pill,
        ]
    }
}

/// Crée un rayon uniforme.
pub const fn radius(r: f32) -> Radius {
    Radius {
        top_left: r,
        top_right: r,
        bottom_right: r,
        bottom_left: r,
    }
}

/// Crée des rayons différents pour chaque coin.
pub const fn radius_custom(
    top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32,
) -> Radius {
    Radius {
        top_left,
        top_right,
        bottom_right,
        bottom_left,
    }
}
