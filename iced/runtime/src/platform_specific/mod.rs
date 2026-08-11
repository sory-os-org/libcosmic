//! Platform specific actions defined for wayland

use std::fmt;

#[cfg(all(feature = "cctk", any(target_os = "linux", target_os = "redox")))]
/// Platform specific actions defined for wayland
pub mod wayland;

/// Platform specific actions defined for wayland
pub enum Action {
    /// Wayland Specific Actions
    #[cfg(all(feature = "cctk", any(target_os = "linux", target_os = "redox")))]
    Wayland(wayland::Action),
}

impl fmt::Debug for Action {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(all(feature = "cctk", any(target_os = "linux", target_os = "redox")))]
            Action::Wayland(action) => action.fmt(_f),
            #[cfg(not(all(feature = "cctk", any(target_os = "linux", target_os = "redox"))))]
            _ => Ok(()),
        }
    }
}
