//! Ombres et lueurs du Design System.

use crate::iced::Color;

/// Descriptif d'ombre portée.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadowToken {
    pub offset_y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
    pub alpha: f32,
}

/// Tokens d'ombre.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadows {
    /// Ombre très subtile (cartes au repos).
    pub sm: ShadowToken,
    /// Ombre standard (cartes survolées, dialogues).
    pub md: ShadowToken,
    /// Ombre forte (modales, popovers).
    pub lg: ShadowToken,
    /// Ombre très forte (menus, toasts).
    pub xl: ShadowToken,
    /// Lueur bleue subtile (glow, focus ring).
    pub glow_sm: ShadowToken,
    /// Lueur bleue moyenne (cartes sélectionnées).
    pub glow_md: ShadowToken,
    /// Lueur bleue forte (boutons primaires).
    pub glow_lg: ShadowToken,
}

/// Ombres dark mode.
pub const SHADOWS_DARK: Shadows = Shadows {
    sm: ShadowToken { offset_y: 1.0, blur: 2.0, spread: 0.0, color: Color::BLACK, alpha: 0.15 },
    md: ShadowToken { offset_y: 4.0, blur: 6.0, spread: -1.0, color: Color::BLACK, alpha: 0.25 },
    lg: ShadowToken { offset_y: 10.0, blur: 15.0, spread: -3.0, color: Color::BLACK, alpha: 0.35 },
    xl: ShadowToken { offset_y: 20.0, blur: 25.0, spread: -5.0, color: Color::BLACK, alpha: 0.45 },
    glow_sm: ShadowToken { offset_y: 0.0, blur: 8.0, spread: 0.0, color: rgb(0x2b6de8), alpha: 0.15 },
    glow_md: ShadowToken { offset_y: 0.0, blur: 16.0, spread: 0.0, color: rgb(0x4a8aff), alpha: 0.25 },
    glow_lg: ShadowToken { offset_y: 0.0, blur: 24.0, spread: 0.0, color: rgb(0x2b6de8), alpha: 0.35 },
};

/// Ombres light mode.
pub const SHADOWS_LIGHT: Shadows = Shadows {
    sm: ShadowToken { offset_y: 1.0, blur: 2.0, spread: 0.0, color: rgb(0x0f172a), alpha: 0.06 },
    md: ShadowToken { offset_y: 4.0, blur: 6.0, spread: -1.0, color: rgb(0x0f172a), alpha: 0.10 },
    lg: ShadowToken { offset_y: 10.0, blur: 15.0, spread: -3.0, color: rgb(0x0f172a), alpha: 0.15 },
    xl: ShadowToken { offset_y: 20.0, blur: 25.0, spread: -5.0, color: rgb(0x0f172a), alpha: 0.20 },
    glow_sm: ShadowToken { offset_y: 0.0, blur: 8.0, spread: 0.0, color: rgb(0x2563eb), alpha: 0.15 },
    glow_md: ShadowToken { offset_y: 0.0, blur: 16.0, spread: 0.0, color: rgb(0x3b82f6), alpha: 0.20 },
    glow_lg: ShadowToken { offset_y: 0.0, blur: 24.0, spread: 0.0, color: rgb(0x2563eb), alpha: 0.30 },
};

impl ShadowToken {
    /// Crée un `iced::Shadow` à partir de ce token.
    pub fn to_iced_shadow(&self) -> iced_core::Shadow {
        iced_core::Shadow {
            color: Color { a: self.alpha / 1.0, ..self.color },
            offset: iced_core::Vector::new(0.0, self.offset_y),
            blur_radius: self.blur,
        }
    }
}

const fn rgb(hex: u32) -> Color {
    Color::from_rgb(
        ((hex >> 16) & 0xff) as f32 / 255.0,
        ((hex >> 8) & 0xff) as f32 / 255.0,
        (hex & 0xff) as f32 / 255.0,
    )
}
