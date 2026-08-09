// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Layouts de page SoryOS — patterns shadcn/Radix.
//!
//! Fournit des conteneurs de mise en page pour applications complexes :
//! - `sory_page_view` : Vue principale avec header, sidebar, contenu et footer
//! - `sory_sub_page` : Sous-page avec navigation retour et titre
//! - `sory_page_container` : Conteneur de contenu principal scrollable
//! - `sory_empty_state` : État vide (aucun contenu)
//! - `sory_loading_state` : État de chargement
//! - `sory_page_header` : Barre d'en-tête de page
//! - `sory_page_stack` : Stack de sous-pages (navigation arrière)

use std::borrow::Cow;

use crate::iced::{Alignment, Length, Padding};

use crate::widget::{column, container, row, scrollable, space, text};
use crate::Element;

// ═════════════════════════════════════════════════════════════════════════════
// PAGE VIEW — Vue principale d'application
// ═════════════════════════════════════════════════════════════════════════════

/// Builder pour une vue de page complète SoryOS.
///
/// Layout : `header | (sidebar + content) | footer`
pub struct SoryPageView<'a, Message> {
    header: Option<Element<'a, Message>>,
    sidebar: Option<Element<'a, Message>>,
    content: Element<'a, Message>,
    footer: Option<Element<'a, Message>>,
    details_panel: Option<Element<'a, Message>>,
    sidebar_width: Length,
    details_width: Length,
    max_sidebar_width: Option<f32>,
    min_content_width: Option<f32>,
    padding: Padding,
    /// Whether to wrap content in a scrollable (default: true).
    scrollable_content: bool,
    /// Custom content padding (None = use theme spacing).
    content_padding: Option<Padding>,
}

impl<'a, Message> SoryPageView<'a, Message> {
    /// Crée une nouvelle vue de page avec le contenu principal obligatoire.
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            header: None,
            sidebar: None,
            content: content.into(),
            footer: None,
            details_panel: None,
            sidebar_width: Length::Fixed(240.0),
            details_width: Length::Fixed(300.0),
            max_sidebar_width: None,
            min_content_width: None,
            padding: Padding::default(),
            scrollable_content: true,
            content_padding: None,
        }
    }

    /// Définit la barre d'en-tête.
    pub fn header(mut self, header: impl Into<Element<'a, Message>>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Définit la sidebar de navigation.
    pub fn sidebar(mut self, sidebar: impl Into<Element<'a, Message>>) -> Self {
        self.sidebar = Some(sidebar.into());
        self
    }

    /// Définit la largeur de la sidebar.
    pub fn sidebar_width(mut self, width: impl Into<Length>) -> Self {
        self.sidebar_width = width.into();
        self
    }

    /// Définit la largeur maximale de la sidebar.
    pub fn max_sidebar_width(mut self, max_width: f32) -> Self {
        self.max_sidebar_width = Some(max_width);
        self
    }

    /// Définit la largeur minimale du contenu.
    pub fn min_content_width(mut self, min_width: f32) -> Self {
        self.min_content_width = Some(min_width);
        self
    }

    /// Définit le padding du contenu.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Définit le padding du contenu principal.
    ///
    /// Si non défini, utilise le spacing du thème.
    pub fn content_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.content_padding = Some(padding.into());
        self
    }

    /// Désactive l'enveloppement scrollable du contenu principal.
    ///
    /// Par défaut, le contenu est wrappé dans un `scrollable`.
    /// Cette méthode permet de le désactiver pour un contenu non-scrollable.
    pub fn no_scrollable(mut self) -> Self {
        self.scrollable_content = false;
        self
    }

    /// Définit le pied de page.
    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Définit le panneau de détails (panneau droit).
    pub fn details_panel(mut self, panel: impl Into<Element<'a, Message>>) -> Self {
        self.details_panel = Some(panel.into());
        self
    }

    /// Définit la largeur du panneau de détails.
    pub fn details_width(mut self, width: impl Into<Length>) -> Self {
        self.details_width = width.into();
        self
    }
}

impl<'a, Message: Clone + 'static> From<SoryPageView<'a, Message>> for Element<'a, Message> {
    fn from(page: SoryPageView<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        // ── Colonne principale (header + body + footer) ───────────────
        let mut main_col = column::with_capacity(3);

        // Header
        if let Some(header) = page.header {
            main_col = main_col.push(header);
        }

        // Body : sidebar + content + details
        let mut body_row = row::with_capacity(3)
            .spacing(0)
            .align_y(Alignment::Start);

        if let Some(sidebar) = page.sidebar {
            let mut sidebar_container = container(sidebar)
                .class(crate::theme::sory::sidebar())
                .width(page.sidebar_width)
                .height(Length::Fill);

            if let Some(max_w) = page.max_sidebar_width {
                sidebar_container = sidebar_container.max_width(max_w);
            }

            body_row = body_row.push(sidebar_container);
        }

        // Contenu principal (optionnellement scrollable)
        let content_element: Element<'_, Message> = if page.scrollable_content {
            scrollable(page.content)
                .height(Length::Fill)
                .class(crate::style::iced::Scrollable::Minimal)
                .into()
        } else {
            page.content
        };

        let content_pad = page.content_padding.unwrap_or(spacing.space_m.into());
        let mut content_container = container(content_element)
            .class(crate::theme::sory::background())
            .padding(content_pad)
            .width(Length::Fill)
            .height(Length::Fill);

        if let Some(min_w) = page.min_content_width {
            content_container = content_container.width(Length::Fixed(min_w));
        }

        if page.padding.top > 0.0 || page.padding.bottom > 0.0 || page.padding.left > 0.0 || page.padding.right > 0.0 {
            content_container = content_container.padding(page.padding);
        }

        body_row = body_row.push(content_container);

        // Panneau de détails
        if let Some(details) = page.details_panel {
            body_row = body_row.push(
                container(details)
                    .class(crate::theme::sory::details_panel())
                    .width(page.details_width)
                    .height(Length::Fill),
            );
        }

        main_col = main_col.push(body_row.height(Length::Fill));

        // Footer
        if let Some(footer) = page.footer {
            main_col = main_col.push(footer);
        }

        container(main_col)
            .class(crate::theme::sory::bg_deep())
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// SUB PAGE — Sous-page avec navigation retour
// ═════════════════════════════════════════════════════════════════════════════

/// Builder pour une sous-page avec bouton retour et titre.
pub struct SorySubPage<'a, Message> {
    title: Element<'a, Message>,
    subtitle: Option<Element<'a, Message>>,
    back_button: Option<Element<'a, Message>>,
    actions: Vec<Element<'a, Message>>,
    content: Element<'a, Message>,
    max_width: Option<f32>,
    centered: bool,
}

impl<'a, Message> SorySubPage<'a, Message> {
    /// Crée une sous-page avec titre et contenu.
    pub fn new(
        title: impl Into<Element<'a, Message>>,
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            back_button: None,
            actions: Vec::new(),
            content: content.into(),
            max_width: None,
            centered: false,
        }
    }

    /// Ajoute un sous-titre.
    pub fn subtitle(mut self, subtitle: impl Into<Element<'a, Message>>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Ajoute un bouton retour.
    pub fn back_button(mut self, button: impl Into<Element<'a, Message>>) -> Self {
        self.back_button = Some(button.into());
        self
    }

    /// Ajoute une action dans la barre de titre.
    pub fn action(mut self, action: impl Into<Element<'a, Message>>) -> Self {
        self.actions.push(action.into());
        self
    }

    /// Définit la largeur maximale du contenu.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Centre le contenu.
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }
}

impl<'a, Message: Clone + 'static> From<SorySubPage<'a, Message>> for Element<'a, Message> {
    fn from(page: SorySubPage<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        // ── Barre de titre avec retour ────────────────────────────────
        let mut header_row = row::with_capacity(4)
            .spacing(spacing.space_s)
            .align_y(Alignment::Center);

        if let Some(back) = page.back_button {
            header_row = header_row.push(back);
        }

        let mut title_col = column::with_capacity(2)
            .push(page.title);
        if let Some(subtitle) = page.subtitle {
            title_col = title_col.push(subtitle);
        }
        header_row = header_row.push(title_col.width(Length::Fill));

        for action in page.actions {
            header_row = header_row.push(action);
        }

        let header = container(header_row)
            .class(crate::theme::sory::header_bar())
            .padding(Padding::from([spacing.space_s, spacing.space_m]))
            .width(Length::Fill);

        // ── Contenu scrollable ────────────────────────────────────────
        let mut content_col = column::with_capacity(1)
            .push(page.content)
            .spacing(spacing.space_m);

        let mut scroll = scrollable(content_col)
            .height(Length::Fill)
            .class(crate::style::iced::Scrollable::Minimal);

        let mut wrapper = column::with_capacity(2)
            .push(header)
            .push(scroll.height(Length::Fill));

        if let Some(max_w) = page.max_width {
            wrapper = wrapper.max_width(max_w);
        }

        let centered = page.centered;

        let mut wrapper_container = container(wrapper)
            .class(crate::theme::sory::background())
            .padding(spacing.space_m)
            .width(Length::Fill)
            .height(Length::Fill);

        if centered {
            wrapper_container = wrapper_container.center_x(Length::Fill);
        }

        wrapper_container.into()
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// PAGE HEADER — Barre d'en-tête de page
// ═════════════════════════════════════════════════════════════════════════════

/// Builder pour une barre d'en-tête de page.
pub struct SoryPageHeader<'a, Message> {
    title: Element<'a, Message>,
    subtitle: Option<Element<'a, Message>>,
    icon: Option<Element<'a, Message>>,
    actions: Vec<Element<'a, Message>>,
    breadcrumb: Option<Element<'a, Message>>,
}

impl<'a, Message> SoryPageHeader<'a, Message> {
    /// Crée une en-tête de page avec titre.
    pub fn new(title: impl Into<Element<'a, Message>>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            icon: None,
            actions: Vec::new(),
            breadcrumb: None,
        }
    }

    /// Ajoute un sous-titre.
    pub fn subtitle(mut self, subtitle: impl Into<Element<'a, Message>>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Ajoute une icône.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Ajoute une action (bouton).
    pub fn action(mut self, action: impl Into<Element<'a, Message>>) -> Self {
        self.actions.push(action.into());
        self
    }

    /// Ajoute un breadcrumb.
    pub fn breadcrumb(mut self, breadcrumb: impl Into<Element<'a, Message>>) -> Self {
        self.breadcrumb = Some(breadcrumb.into());
        self
    }
}

impl<'a, Message: Clone + 'static> From<SoryPageHeader<'a, Message>> for Element<'a, Message> {
    fn from(header: SoryPageHeader<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        let mut col = column::with_capacity(3);

        // Breadcrumb
        if let Some(bc) = header.breadcrumb {
            col = col.push(bc);
        }

        // Ligne titre + actions
        let mut title_row = row::with_capacity(3)
            .spacing(spacing.space_m)
            .align_y(Alignment::Center);

        if let Some(icon) = header.icon {
            title_row = title_row.push(icon);
        }

        let mut title_col = column::with_capacity(2)
            .push(header.title);
        if let Some(subtitle) = header.subtitle {
            title_col = title_col.push(subtitle);
        }
        title_row = title_row.push(title_col.width(Length::Fill));

        for action in header.actions {
            title_row = title_row.push(action);
        }

        col = col.push(title_row);

        container(col)
            .padding(Padding::from([
                spacing.space_m,
                spacing.space_m,
                spacing.space_s,
                spacing.space_m,
            ]))
            .width(Length::Fill)
            .into()
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// PAGE STACK — Navigation arrière pour sous-pages
// ═════════════════════════════════════════════════════════════════════════════

/// Stack de pages pour la navigation arrière.
///
/// Maintient un historique de pages et permet de naviguer retour.
pub struct SoryPageStack<'a, Message> {
    pages: Vec<Element<'a, Message>>,
    current: usize,
    on_back: Option<Message>,
}

impl<'a, Message: Clone + 'static> SoryPageStack<'a, Message> {
    /// Crée un nouveau stack de pages.
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            current: 0,
            on_back: None,
        }
    }

    /// Ajoute une page au stack.
    pub fn page(mut self, page: impl Into<Element<'a, Message>>) -> Self {
        self.pages.push(page.into());
        self
    }

    /// Définit l'index de la page actuelle.
    pub fn current(mut self, index: usize) -> Self {
        self.current = index.min(self.pages.len().saturating_sub(1));
        self
    }

    /// Définit le message de navigation arrière.
    pub fn on_back(mut self, msg: Message) -> Self {
        self.on_back = Some(msg);
        self
    }
}

impl<'a, Message: Clone + 'static> From<SoryPageStack<'a, Message>> for Element<'a, Message> {
    fn from(stack: SoryPageStack<'a, Message>) -> Self {
        if let Some(page) = stack.pages.into_iter().nth(stack.current) {
            page
        } else {
            // Fallback : état vide
            container(
                text::body("Aucune page")
                    .center()
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// PAGE CONTAINER — Conteneur de contenu principal
// ═════════════════════════════════════════════════════════════════════════════

/// Crée un conteneur de contenu principal scrollable avec padding SoryOS.
pub fn sory_page_content<'a, Message: Clone + 'static>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();
    let col = column::with_capacity(1)
        .push(content.into())
        .spacing(spacing.space_m);

    container(
        scrollable(col)
            .height(Length::Fill)
            .class(crate::style::iced::Scrollable::Minimal),
    )
    .class(crate::theme::sory::background())
    .padding(spacing.space_m)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Crée un conteneur de section avec titre et séparateur.
pub fn sory_section<'a, Message: Clone + 'static>(
    title: impl Into<Cow<'a, str>> + 'a,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let mut col = column::with_capacity(3)
        .spacing(spacing.space_s)
        .push(text::heading(title))
        .push(
            container(space::horizontal())
                .height(1.0)
                .width(Length::Fill)
                .class(crate::theme::sory::sidebar_separator()),
        )
        .push(content.into());

    col.into()
}

/// Crée un conteneur de grille responsive (nombre de colonnes variable).
pub fn sory_grid<'a, Message: Clone + 'static>(
    columns: usize,
    spacing_val: f32,
    items: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut rows = column::with_capacity(items.len() / columns + 1).spacing(spacing_val);

    let mut iter = items.into_iter().peekable();
    while iter.peek().is_some() {
        let mut row = row::with_capacity(columns).spacing(spacing_val);
        for _ in 0..columns {
            if let Some(item) = iter.next() {
                row = row.push(container(item).width(Length::Fill));
            } else {
                row = row.push(space::horizontal().width(Length::Fill));
            }
        }
        rows = rows.push(row);
    }

    rows.into()
}

// ═════════════════════════════════════════════════════════════════════════════
// EMPTY STATE — État vide
// ═════════════════════════════════════════════════════════════════════════════

/// Crée un état vide avec icône, titre et description.
pub fn sory_empty_state<'a, Message: Clone + 'static>(
    icon: impl Into<Element<'a, Message>>,
    title: impl Into<Cow<'a, str>> + 'a,
    description: impl Into<Cow<'a, str>> + 'a,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let content = column::with_capacity(3)
        .spacing(spacing.space_m)
        .align_x(Alignment::Center)
        .push(
            container(icon.into())
                .class(crate::theme::sory::empty_state())
                .padding(24)
        )
        .push(
            text::title3(title)
                .center()
                .width(Length::Fill),
        )
        .push(
            text::body(description)
                .center()
                .width(Length::Fill),
        );

    container(content)
        .class(crate::theme::sory::empty_state())
        .padding(spacing.space_xl)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

/// Crée un état vide avec une action (bouton).
pub fn sory_empty_state_with_action<'a, Message: Clone + 'static>(
    icon: impl Into<Element<'a, Message>>,
    title: impl Into<Cow<'a, str>> + 'a,
    description: impl Into<Cow<'a, str>> + 'a,
    action: Element<'a, Message>,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let content = column::with_capacity(4)
        .spacing(spacing.space_m)
        .align_x(Alignment::Center)
        .push(
            container(icon.into())
                .class(crate::theme::sory::empty_state())
                .padding(24)
        )
        .push(
            text::title3(title)
                .center()
                .width(Length::Fill),
        )
        .push(
            text::body(description)
                .center()
                .width(Length::Fill),
        )
        .push(action);

    container(content)
        .class(crate::theme::sory::empty_state())
        .padding(spacing.space_xl)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

// ═════════════════════════════════════════════════════════════════════════════
// LOADING STATE — État de chargement
// ═════════════════════════════════════════════════════════════════════════════

/// Crée un état de chargement avec spinner et message.
pub fn sory_loading_state<'a, Message: Clone + 'static>(
    message: impl Into<Cow<'a, str>> + 'a,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let content = column::with_capacity(2)
        .spacing(spacing.space_m)
        .align_x(Alignment::Center)
        .push(
            // Spinner placeholder (cercle animé)
            container(space::horizontal())
                .width(48.0)
                .height(48.0)
                .class(crate::theme::sory::loading_state()),
        )
        .push(
            text::body(message)
                .center()
                .width(Length::Fill),
        );

    container(content)
        .padding(spacing.space_xl)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

/// Crée un skeleton loader (rectangle animé).
pub fn sory_skeleton<'a, Message: Clone + 'static>(
    width: Length,
    height: f32,
) -> Element<'a, Message> {
    container(space::horizontal())
        .width(width)
        .height(height)
        .class(crate::theme::sory::loading_state())
        .into()
}
