//! Composants modernes SoryOS — design system complet avec animations.
//!
//! Chaque composant utilise :
//! - Les **design tokens** (`crate::design`) pour des couleurs/espaces cohérents
//! - Les **animations spring** (`crate::anim::spring`) pour des micro-interactions fluides
//! - Le **wrapper Animated** (`crate::widget::anim`) pour les effets hover/press
//! - Le **moteur de style** (`crate::style`) pour une personnalisation facile

pub mod avatar;
pub mod badge;
pub mod breadcrumbs;
pub mod button;
pub mod card;
pub mod dialog;
pub mod divider;
pub mod dropdown;
pub mod progress;
pub mod search_bar;
pub mod skeleton;
pub mod switch;
pub mod table;
pub mod tabs;

pub use avatar::{AvatarSize, ModernAvatar};
pub use badge::{BadgeVariant, ModernBadge};
pub use breadcrumbs::Breadcrumbs;
pub use button::{ButtonVariant, ModernButton};
pub use card::ModernCard;
pub use dialog::ModernDialog;
pub use divider::ModernDivider;
pub use dropdown::ModernDropdown;
pub use progress::ModernProgressBar;
pub use search_bar::ModernSearchBar;
pub use skeleton::ModernSkeleton;
pub use switch::ModernSwitch;
pub use table::{ModernTable, TableColumn};
pub use tabs::{ModernTabs, TabStyle};
