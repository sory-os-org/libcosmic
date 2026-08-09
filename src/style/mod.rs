//! Système de styles libcosmic.
//!
//! Ce module ré-exporte le système de `theme::style` pour la compatibilité
//! avec le code existant, et ajoute le sous-module `css` qui implémente
//! un moteur de styles CSS natif pour les widgets SoryOS.

pub mod css;

// ── Ré-exports depuis theme::style pour compatibilité ────────────────
pub use crate::theme::style::*;
