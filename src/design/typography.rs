//! Typographie du Design System.

/// Tokens de typographie.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Typography {
    /// Police par défaut pour l'interface.
    pub font_family: &'static str,
    /// Police pour le code (monospace).
    pub font_family_mono: &'static str,
    /// Taille de police de base.
    pub font_size_base: f32,
    /// Échelle de taille.
    pub h1: f32,
    pub h2: f32,
    pub h3: f32,
    pub h4: f32,
    pub body: f32,
    pub body_small: f32,
    pub caption: f32,
    pub overline: f32,
    /// Poids de police.
    pub weight_light: u16,
    pub weight_regular: u16,
    pub weight_medium: u16,
    pub weight_semibold: u16,
    pub weight_bold: u16,
    /// Hauteur de ligne.
    pub line_height_tight: f32,
    pub line_height_normal: f32,
    pub line_height_relaxed: f32,
}

pub const TYPOGRAPHY: Typography = Typography {
    font_family: "",
    font_family_mono: "",
    font_size_base: 14.0,
    h1: 28.0,
    h2: 22.0,
    h3: 18.0,
    h4: 16.0,
    body: 14.0,
    body_small: 13.0,
    caption: 12.0,
    overline: 11.0,
    weight_light: 300,
    weight_regular: 400,
    weight_medium: 500,
    weight_semibold: 600,
    weight_bold: 700,
    line_height_tight: 1.2,
    line_height_normal: 1.5,
    line_height_relaxed: 1.75,
};
