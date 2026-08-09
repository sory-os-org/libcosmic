//! Moteur de layout moderne SoryOS — Flex, Grid, Responsive, Dock, SplitPane, Stack.
//!
//! Fournit des conteneurs de layout intelligents et flexibles :
//! - `Flex` : Row/Column avec alignement, justification, wrap
//! - `Grid` : Grille avec spanning
//! - `Dock` : 5 régions (N/S/E/W/Center)
//! - `SplitPane` : Split horizontal/vertical avec ratio
//! - `Stack` : Overlay / z-index / positionnement relatif
//! - `Responsive` : Layout adaptatif selon la taille disponible
//!
//! Tous les conteneurs respectent les `Length` iced (Fill, Shrink, Fixed, FillPortion)
//! et n'imposent aucune dimension forcée.

pub mod dock;
pub mod flex;
pub mod grid;
pub mod responsive;
pub mod split_pane;
pub mod stack;

pub use dock::{Dock, DockRegion};
pub use flex::{Flex, FlexAlign, FlexDirection, FlexJustify};
pub use grid::{Grid, GridSpan};
pub use responsive::{Breakpoint, responsive_layout};
pub use split_pane::{Axis, SplitPane};
pub use stack::Stack;
