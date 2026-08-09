//! Espacements du Design System — échelle cohérente.
//!
//! Basée sur une échelle 4px (taille de base).

use iced_core::Padding;

/// Tokens d'espacement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spacing {
    /// 0px
    pub none: f32,
    /// 2px — espacement minimal
    pub px: f32,
    /// 4px
    pub xxs: f32,
    /// 6px
    pub xs: f32,
    /// 8px
    pub sm: f32,
    /// 12px
    pub md: f32,
    /// 16px
    pub lg: f32,
    /// 20px
    pub xl: f32,
    /// 24px
    pub xxl: f32,
    /// 32px
    pub xxxl: f32,
    /// 40px
    pub xxxxl: f32,
    /// 48px
    pub section: f32,
    /// 64px
    pub page: f32,
}

/// Échelle d'espacement par défaut (base 4px).
pub const SPACING: Spacing = Spacing {
    none: 0.0,
    px: 2.0,
    xxs: 4.0,
    xs: 6.0,
    sm: 8.0,
    md: 12.0,
    lg: 16.0,
    xl: 20.0,
    xxl: 24.0,
    xxxl: 32.0,
    xxxxl: 40.0,
    section: 48.0,
    page: 64.0,
};

/// Crée un padding uniforme à partir d'un token d'espacement.
pub fn padding(value: f32) -> Padding {
    Padding::new(value)
}

/// Crée un padding horizontal/vertical.
pub fn padding_xy(x: f32, y: f32) -> Padding {
    Padding::from([y, x])
}

/// Crée un padding avec des valeurs différentes pour chaque côté.
pub fn padding_tb(t: f32, r: f32, b: f32, l: f32) -> Padding {
    Padding::from([t, r, b, l])
}

impl Spacing {
    pub const fn as_slice(&self) -> [f32; 13] {
        [
            self.none, self.px, self.xxs, self.xs, self.sm, self.md,
            self.lg, self.xl, self.xxl, self.xxxl, self.xxxxl,
            self.section, self.page,
        ]
    }
}
