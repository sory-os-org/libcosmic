//! # SoryOS Design System — Guide d'utilisation
//!
//! Ce fichier montre comment utiliser le design system SoryOS avec :
//! - Le moteur CSS natif pour les styles déclaratifs
//! - Les composants modernes (buttons, cards, tabs, table, etc.)
//! - Les animations spring pour les micro-interactions
//! - Les layouts sidebar+content
//!
//! ## Installation
//!
//! Ajoutez dans votre `main.rs`:
//!
//! ```ignore
//! use cosmic::style::css;
//!
//! // Au début de votre application:
//! css::load_soryos_theme(true); // dark mode
//! // ou
//! css::load_soryos_theme(false); // light mode
//! ```
//!
//! ## Composants disponibles
//!
//! ### Boutons modernes
//! ```ignore
//! use cosmic::widget::modern::*;
//!
//! let primary_btn = ModernButton::new("Primary")
//!     .variant(ButtonVariant::Primary);
//!
//! let secondary_btn = ModernButton::new("Secondary")
//!     .variant(ButtonVariant::Secondary);
//!
//! let ghost_btn = ModernButton::new("Ghost")
//!     .variant(ButtonVariant::Ghost);
//! ```
//!
//! ### Cartes
//! ```ignore
//! let card = ModernCard::new(content)
//!     .on_press(Message::CardClicked);
//! ```
//!
//! ### Onglets
//! ```ignore
//! let tabs = ModernTabs::new()
//!     .style(TabStyle::Underline)
//!     .push_tab("Tab 1", active_tab == 0, Message::Tab1)
//!     .push_tab("Tab 2", active_tab == 1, Message::Tab2);
//! ```
//!
//! ### Table
//! ```ignore
//! let table = ModernTable::new()
//!     .column(TableColumn::new("Nom"))
//!     .column(TableColumn::new("Taille"))
//!     .row(TableRow::new().push_cell("fichier.txt").push_cell("12 KB"))
//!     .row(TableRow::new().push_cell("image.png").push_cell("2.4 MB"));
//! ```
//!
//! ### Breadcrumbs
//! ```ignore
//! let breadcrumbs = Breadcrumbs::new()
//!     .push_home(Message::Home)
//!     .push_item("Documents", Some(Message::Docs))
//!     .push_item("Projet", None);
//! ```
//!
//! ### Avatar
//! ```ignore
//! let avatar = ModernAvatar::new("John Doe")
//!     .size(AvatarSize::LG)
//!     .online(true);
//! ```
//!
//! ### Switch / Toggle
//! ```ignore
//! let toggle = ModernSwitch::new()
//!     .label("Mode sombre")
//!     .toggled(is_dark)
//!     .on_toggle(Message::ToggleDark);
//! ```
//!
//! ### Dropdown
//! ```ignore
//! let dropdown = ModernDropdown::new("Choisir une option")
//!     .push_item("Option 1", Message::Option1)
//!     .push_item("Option 2", Message::Option2)
//!     .push_separator()
//!     .push_danger_item("Supprimer", Message::Delete);
//! ```
//!
//! ### Search Bar
//! ```ignore
//! let search = ModernSearchBar::new("Rechercher...");
//! ```
//!
//! ## Utiliser le CSS natif
//!
//! ### Styles pour containers
//! ```ignore
//! use cosmic::style::css;
//!
//! // Appliquer des styles CSS à un container
//! let styled_container = container(content)
//!     .class(css::apply_css("Card", &["elevated"]));
//!
//! // Ou avec des classes multiples
//! let sidebar = container(nav)
//!     .class(css::apply_css("Sidebar", &["nav-item", "active"]));
//! ```
//!
//! ### Styles personnalisés
//! ```ignore
//! // Charger du CSS personnalisé
//! css::load_css_string("custom", r#"
//!     .my-widget {
//!         background: #ff0000;
//!         border-radius: 8px;
//!     }
//! "#);
//!
//! // Utiliser
//! let widget = container(content)
//!     .class(css::apply_css("Container", &["my-widget"]));
//! ```
//!
//! ### Hot-reload des fichiers CSS
//! ```ignore
//! // Charger depuis un fichier
//! css::load_css_file("theme", "assets/styles/main.css").unwrap();
//!
//! // Recharger les fichiers modifiés
//! let changed = css::reload_css_files();
//! if !changed.is_empty() {
//!     println!("Fichiers rechargés: {:?}", changed);
//! }
//! ```
//!
//! ## Layout patterns
//!
//! ### App Shell (Sidebar + Content)
//! ```ignore
//! // Utiliser les classes CSS layout
//! let app = container(
//!     row::with_capacity(2)
//!         .push(sidebar)
//!         .push(content)
//! )
//! .class(css::apply_css("Container", &["app-shell"]));
//! ```
//!
//! ### Grid Layout
//! ```ignore
//! let grid = container(
//!     row::with_capacity(3)
//!         .spacing(16)
//!         .push(card1)
//!         .push(card2)
//!         .push(card3)
//! )
//! .class(css::apply_css("Container", &["grid-3"]));
//! ```
//!
//! ## Animations
//!
//! ### Hover effect avec spring
//! ```ignore
//! use cosmic::widget::anim::Animated;
//!
//! let animated_card = Animated::new(card_content)
//!     .on_hover(Message::CardHover)
//!     .on_press(Message::CardPress);
//! ```
//!
//! ### Presets d'animation
//! ```ignore
//! use cosmic::widget::anim::AnimPreset;
//!
//! let lift_card = Animated::with_preset(card, AnimPreset::Lift);
//! let glow_card = Animated::with_preset(card, AnimPreset::Glow);
//! ```
//!
//! ## Exemples complets
//!
//! Voir `examples/soryos_app.rs` pour un exemple complet d'application
//! utilisant tous ces composants ensemble.
