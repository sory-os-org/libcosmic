//! Tokens de motion — durées, easing, et presets d'animation.
//!
//! Inspiré de Material Design 3 Motion System et de Framer Motion.

use crate::anim::easing::Easing;
use crate::anim::spring::SpringConfig;
use std::time::Duration;

/// Tokens de durée pour les animations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DurationTokens {
    /// Micro-interactions (hover, press) — 50ms
    pub micro: Duration,
    /// Feedback UI rapide (toggle, tooltip) — 100ms
    pub fastest: Duration,
    /// Transitions simples (dropdown, focus) — 150ms
    pub faster: Duration,
    /// Transitions standard — 200ms
    pub fast: Duration,
    /// Animations modérées — 300ms
    pub normal: Duration,
    /// Apparitions d'éléments (modale, drawer) — 400ms
    pub slow: Duration,
    /// Transitions complexes (page) — 500ms
    pub slower: Duration,
    /// Animations expressives (entrée en scène) — 700ms
    pub slowest: Duration,
    /// Animations décoratives (fond, glow) — 1000ms
    pub decorative: Duration,
}

/// Tokens de durée par défaut.
pub const DURATIONS: DurationTokens = DurationTokens {
    micro: Duration::from_millis(50),
    fastest: Duration::from_millis(100),
    faster: Duration::from_millis(150),
    fast: Duration::from_millis(200),
    normal: Duration::from_millis(300),
    slow: Duration::from_millis(400),
    slower: Duration::from_millis(500),
    slowest: Duration::from_millis(700),
    decorative: Duration::from_millis(1000),
};

/// Presets de transitions easing.
pub mod easing {
    use super::Easing;
    /// Accélération rapide, décélération lente — standard Material.
    pub const STANDARD: Easing = Easing::CubicInOut;
    /// Décélération uniquement — pour les entrées.
    pub const DECELERATION: Easing = Easing::CubicOut;
    /// Accélération uniquement — pour les sorties.
    pub const ACCELERATION: Easing = Easing::CubicIn;
    /// Ressort avec léger dépassement (overshoot).
    pub const OVERSHOOT: Easing = Easing::BackOut;
    /// Rebond élastique.
    pub const BOUNCE: Easing = Easing::BounceOut;
    /// Mouvement naturel sinusoïdal.
    pub const SMOOTH: Easing = Easing::SineInOut;
    /// Sortie rapide avec overshoot minimal.
    pub const SHARP: Easing = Easing::QuintOut;
    /// Linéaire pour les animations de rotation continue.
    pub const LINEAR: Easing = Easing::Linear;
}

/// Presets de configuration spring (physique).
pub mod spring {
    use super::SpringConfig;
    /// Rapide et précis — pour les boutons, micro-interactions.
    pub const SNAPPY: SpringConfig = SpringConfig::SNAPPY;
    /// Doux et naturel — pour les cartes, listes.
    pub const GENTLE: SpringConfig = SpringConfig::GENTLE;
    /// Avec rebond — pour les apparitions ludiques.
    pub const WOBBLY: SpringConfig = SpringConfig::WOBBLY;
    /// Ferme et rapide — pour le drag & drop.
    pub const STIFF: SpringConfig = SpringConfig::STIFF;
    /// Pour les modales et dialogues.
    pub const MODAL: SpringConfig = SpringConfig::MODAL;
    /// Pour les slides et panneaux latéraux.
    pub const SLIDE: SpringConfig = SpringConfig::SLIDE;
    /// Sans oscillation — amortissement critique.
    pub const CRITICAL: SpringConfig = SpringConfig::CRITICAL;
}

/// Types d'animation de transition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransitionType {
    /// Apparition en fondu.
    Fade,
    /// Apparition par glissement.
    Slide { from: SlideDirection },
    /// Apparition avec zoom.
    Scale { from: f32 },
    /// Apparition par soulèvement.
    Lift,
}

/// Direction de glissement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlideDirection {
    Up, Down, Left, Right,
}

impl TransitionType {
    pub fn duration(&self) -> Duration {
        match self {
            Self::Fade => DURATIONS.fast,
            Self::Slide { .. } => DURATIONS.normal,
            Self::Scale { .. } => DURATIONS.normal,
            Self::Lift => DURATIONS.fast,
        }
    }

    pub fn easing(&self) -> Easing {
        match self {
            Self::Fade => easing::DECELERATION,
            Self::Slide { .. } => easing::STANDARD,
            Self::Scale { .. } => easing::DECELERATION,
            Self::Lift => easing::DECELERATION,
        }
    }
}
