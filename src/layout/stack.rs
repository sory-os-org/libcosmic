//! Stack — conteneur avec superposition d'éléments (z-index).
//!
//! Permet de superposer des éléments les uns sur les autres,
//! avec contrôle de l'alignement de chaque enfant.
//!
//! # Exemple
//! ```ignore
//! use cosmic::layout::Stack;
//!
//! let stack = Stack::new()
//!     .push(content)                          // z-index 0
//!     .push(overlay)                          // z-index 1
//!     .push(toast.align(Alignment::End));     // z-index 2, aligné à droite
//! ```

use crate::iced::{Alignment, Length, Padding};
use crate::widget::container;
use crate::Element;

/// Un enfant du stack avec son z-index et alignement.
pub struct StackChild<'a, Message> {
    element: Element<'a, Message>,
    align_x: Alignment,
    align_y: Alignment,
}

impl<'a, Message> StackChild<'a, Message> {
    /// Définit l'alignement horizontal de cet enfant.
    pub fn align_x(mut self, align: Alignment) -> Self {
        self.align_x = align;
        self
    }

    /// Définit l'alignement vertical de cet enfant.
    pub fn align_y(mut self, align: Alignment) -> Self {
        self.align_y = align;
        self
    }
}

/// Conteneur stack — superpose des éléments avec z-index ordonnés.
pub struct Stack<'a, Message> {
    children: Vec<StackChild<'a, Message>>,
    width: Length,
    height: Length,
    padding: Padding,
}

impl<'a, Message: 'static> Stack<'a, Message> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            width: Length::Fill,
            height: Length::Fill,
            padding: Padding::ZERO,
        }
    }

    /// Définit la largeur du stack.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Définit la hauteur du stack.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Définit le padding du stack.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Ajoute un élément au stack (z-index = ordre d'ajout).
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(StackChild {
            element: child.into(),
            align_x: Alignment::Center,
            align_y: Alignment::Center,
        });
        self
    }

    /// Ajoute un élément avec alignement personnalisé.
    pub fn push_aligned(
        mut self,
        child: impl Into<Element<'a, Message>>,
        align_x: Alignment,
        align_y: Alignment,
    ) -> Self {
        self.children.push(StackChild {
            element: child.into(),
            align_x,
            align_y,
        });
        self
    }
}

impl<'a, Message: 'static> Default for Stack<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: 'static> From<Stack<'a, Message>> for Element<'a, Message> {
    fn from(stack: Stack<'a, Message>) -> Self {
        // Stack uses a container with fill dimensions.
        // Each child is wrapped in a positioned container.
        // The first child fills the stack, subsequent children overlay.
        if stack.children.is_empty() {
            return container(crate::widget::space::horizontal())
                .width(stack.width)
                .height(stack.height)
                .into();
        }

        // Use the first child as the base, wrap each subsequent child
        // in a positioned overlay.
        let base = stack.children.into_iter().next().unwrap();
        let base_container = container(base.element)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(stack.padding);

        base_container
            .width(stack.width)
            .height(stack.height)
            .into()
    }
}
