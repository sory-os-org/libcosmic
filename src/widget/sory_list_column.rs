// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Widgets de liste stylisés SoryOS.
//!
//! Fournit des wrappers autour de `ListColumn` de libcosmic
//! avec le style SoryOS appliqué automatiquement.

use crate::widget::{self, list_column, settings};
use crate::theme;

/// Crée une `ListColumn` stylisée SoryOS.
#[must_use]
pub fn sory_list_column<'a, Message: Clone + 'static>() -> widget::ListColumn<'a, Message> {
    sory_list_column_with_capacity(4)
}

/// Crée une `ListColumn` stylisée SoryOS avec une capacité donnée.
#[must_use]
pub fn sory_list_column_with_capacity<'a, Message: Clone + 'static>(
    capacity: usize,
) -> widget::ListColumn<'a, Message> {
    list_column::with_capacity(capacity)
        .style(theme::sory::section())
        .list_item_padding([14, 20])
        .divider_padding(20)
}

/// Crée une `ListColumn` stylisée SoryOS pour le contexte/drawer.
#[must_use]
pub fn sory_context_list_column<'a, Message: Clone + 'static>() -> widget::ListColumn<'a, Message> {
    sory_context_list_column_with_capacity(4)
}

/// Crée une `ListColumn` stylisée SoryOS pour le contexte avec une capacité donnée.
#[must_use]
pub fn sory_context_list_column_with_capacity<'a, Message: Clone + 'static>(
    capacity: usize,
) -> widget::ListColumn<'a, Message> {
    list_column::with_capacity(capacity)
        .style(theme::sory::context_content())
        .list_item_padding([9, 14])
        .divider_padding(14)
}

/// Crée une `Section` de réglages stylisée SoryOS.
#[must_use]
pub fn sory_settings_section<'a, Message: Clone + 'static>() -> settings::Section<'a, Message> {
    settings::section::with_column(sory_list_column_with_capacity(4))
}
