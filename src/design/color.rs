//! Couleurs du Design System — palette dynamique et sémantique.
//!
//! Structure hiérarchique :
//! - **Palette** : couleurs brutes (primary, neutrals, accents)
//! - **SemanticColor** : couleurs par rôle (surface, text, border, status)
//! - **ColorMode** : dark/light variants
//! - **DynamicColor** : couleur qui s'adapte au mode

use crate::iced::Color;

/// Ensemble complet de couleurs pour un mode (dark/light).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub primary: PrimaryPalette,
    pub neutral: NeutralPalette,
    pub accent: AccentPalette,
    pub status: StatusPalette,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimaryPalette {
    pub base: Color,
    pub bright: Color,
    pub light: Color,
    pub dark: Color,
    pub muted: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NeutralPalette {
    pub step_0: Color,
    pub step_50: Color,
    pub step_100: Color,
    pub step_200: Color,
    pub step_300: Color,
    pub step_400: Color,
    pub step_500: Color,
    pub step_600: Color,
    pub step_700: Color,
    pub step_800: Color,
    pub step_900: Color,
    pub step_950: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccentPalette {
    pub blue: Color,
    pub purple: Color,
    pub green: Color,
    pub orange: Color,
    pub red: Color,
    pub cyan: Color,
    pub pink: Color,
    pub yellow: Color,
    pub teal: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatusPalette {
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub pending: Color,
}

/// Palette dark mode — Deep Navy Glass
pub const DARK: Palette = Palette {
    primary: PrimaryPalette {
        base: rgb(0x2b6de8),
        bright: rgb(0x4a8aff),
        light: rgb(0x6ba3ff),
        dark: rgb(0x1a3d8f),
        muted: rgb(0x0f1f4a),
    },
    neutral: NeutralPalette {
        step_0: rgb(0x030712),
        step_50: rgb(0x05091b),
        step_100: rgb(0x080e24),
        step_200: rgb(0x0a1229),
        step_300: rgb(0x0d1530),
        step_400: rgb(0x0e1a35),
        step_500: rgb(0x111d3a),
        step_600: rgb(0x162145),
        step_700: rgb(0x1c2955),
        step_800: rgb(0x23316a),
        step_900: rgb(0x2d3d80),
        step_950: rgb(0x3a4d96),
    },
    accent: AccentPalette {
        blue: rgb(0x2b6de8),
        purple: rgb(0x7c5bf5),
        green: rgb(0x22c55e),
        orange: rgb(0xf59e0b),
        red: rgb(0xef4444),
        cyan: rgb(0x06b6d4),
        pink: rgb(0xec4899),
        yellow: rgb(0xeab308),
        teal: rgb(0x14b8a6),
    },
    status: StatusPalette {
        success: rgb(0x22c55e),
        warning: rgb(0xf59e0b),
        error: rgb(0xef4444),
        info: rgb(0x3b82f6),
        pending: rgb(0x8b5cf6),
    },
};

/// Palette light mode — adaptée pour un thème clair
pub const LIGHT: Palette = Palette {
    primary: PrimaryPalette {
        base: rgb(0x2563eb),
        bright: rgb(0x3b82f6),
        light: rgb(0x60a5fa),
        dark: rgb(0x1d4ed8),
        muted: rgb(0xdbeafe),
    },
    neutral: NeutralPalette {
        step_0: rgb(0xffffff),
        step_50: rgb(0xf8fafc),
        step_100: rgb(0xf1f5f9),
        step_200: rgb(0xe2e8f0),
        step_300: rgb(0xcbd5e1),
        step_400: rgb(0x94a3b8),
        step_500: rgb(0x64748b),
        step_600: rgb(0x475569),
        step_700: rgb(0x334155),
        step_800: rgb(0x1e293b),
        step_900: rgb(0x0f172a),
        step_950: rgb(0x020617),
    },
    accent: AccentPalette {
        blue: rgb(0x2563eb),
        purple: rgb(0x7c3aed),
        green: rgb(0x16a34a),
        orange: rgb(0xea580c),
        red: rgb(0xdc2626),
        cyan: rgb(0x0891b2),
        pink: rgb(0xdb2777),
        yellow: rgb(0xca8a04),
        teal: rgb(0x0d9488),
    },
    status: StatusPalette {
        success: rgb(0x16a34a),
        warning: rgb(0xca8a04),
        error: rgb(0xdc2626),
        info: rgb(0x2563eb),
        pending: rgb(0x7c3aed),
    },
};

/// Couleur sémantique — représentée par un token utilisable dans les styles.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SemanticColor {
    Primary,
    PrimaryBright,
    PrimaryLight,
    PrimaryDark,
    PrimaryMuted,
    Background,
    Surface,
    Elevated,
    Modal,
    Text,
    TextSecondary,
    TextMuted,
    TextOnAccent,
    Border,
    BorderActive,
    BorderSubtle,
    Success,
    Warning,
    Error,
    Info,
    Pending,
    AccentBlue,
    AccentPurple,
    AccentGreen,
    AccentOrange,
    AccentRed,
    AccentCyan,
    AccentPink,
    Transparent,
    Custom(Color),
}

impl SemanticColor {
    pub fn resolve(&self, palette: &Palette) -> Color {
        match self {
            Self::Primary => palette.primary.base,
            Self::PrimaryBright => palette.primary.bright,
            Self::PrimaryLight => palette.primary.light,
            Self::PrimaryDark => palette.primary.dark,
            Self::PrimaryMuted => palette.primary.muted,
            Self::Background => palette.neutral.step_0,
            Self::Surface => palette.neutral.step_100,
            Self::Elevated => palette.neutral.step_200,
            Self::Modal => palette.neutral.step_300,
            Self::Text => palette.neutral.step_950,
            Self::TextSecondary => palette.neutral.step_500,
            Self::TextMuted => palette.neutral.step_400,
            Self::TextOnAccent => palette.neutral.step_0,
            Self::Border => palette.neutral.step_300,
            Self::BorderActive => palette.primary.base,
            Self::BorderSubtle => palette.neutral.step_200,
            Self::Success => palette.status.success,
            Self::Warning => palette.status.warning,
            Self::Error => palette.status.error,
            Self::Info => palette.status.info,
            Self::Pending => palette.status.pending,
            Self::AccentBlue => palette.accent.blue,
            Self::AccentPurple => palette.accent.purple,
            Self::AccentGreen => palette.accent.green,
            Self::AccentOrange => palette.accent.orange,
            Self::AccentRed => palette.accent.red,
            Self::AccentCyan => palette.accent.cyan,
            Self::AccentPink => palette.accent.pink,
            Self::Transparent => Color::TRANSPARENT,
            Self::Custom(c) => *c,
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
