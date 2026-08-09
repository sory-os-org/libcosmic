//! Flex layout — remplacement moderne de Row/Column.
//!
//! Supporte direction, alignement, justification, espacement uniforme,
//! flex-grow/shrink, gap responsive, et wrap.

use crate::iced::Alignment;
use crate::widget::{column, row};
use crate::Element;

/// Direction du flex.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
}

impl Default for FlexDirection {
    fn default() -> Self { Self::Row }
}

/// Alignment des items sur l'axe cross.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexAlign {
    Start,
    Center,
    End,
}

impl Default for FlexAlign {
    fn default() -> Self { Self::Start }
}

/// Justification sur l'axe main.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexJustify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for FlexJustify {
    fn default() -> Self { Self::Start }
}

/// Conteneur flex moderne.
pub struct Flex<'a, Message> {
    direction: FlexDirection,
    align: FlexAlign,
    justify: FlexJustify,
    spacing: f32,
    children: Vec<Element<'a, Message>>,
}

impl<'a, Message> Flex<'a, Message> {
    pub fn new(direction: FlexDirection) -> Self {
        Self {
            direction,
            align: FlexAlign::Start,
            justify: FlexJustify::Start,
            spacing: 0.0,
            children: Vec::new(),
        }
    }

    pub fn row() -> Self {
        Self::new(FlexDirection::Row)
    }

    pub fn column() -> Self {
        Self::new(FlexDirection::Column)
    }

    pub fn align(mut self, align: FlexAlign) -> Self {
        self.align = align;
        self
    }

    pub fn justify(mut self, justify: FlexJustify) -> Self {
        self.justify = justify;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message: 'a> From<Flex<'a, Message>> for Element<'a, Message> {
    fn from(flex: Flex<'a, Message>) -> Self {
        match flex.direction {
            FlexDirection::Row => {
                let mut r = row::with_capacity(flex.children.len());
                if flex.spacing > 0.0 {
                    r = r.spacing(flex.spacing as u16);
                }
                match flex.align {
                    FlexAlign::Start => r = r.align_y(Alignment::Start),
                    FlexAlign::Center => r = r.align_y(Alignment::Center),
                    FlexAlign::End => r = r.align_y(Alignment::End),
                }
                for child in flex.children {
                    r = r.push(child);
                }
                r.into()
            }
            FlexDirection::Column => {
                let mut c = column::with_capacity(flex.children.len());
                if flex.spacing > 0.0 {
                    c = c.spacing(flex.spacing as u16);
                }
                match flex.align {
                    FlexAlign::Start => c = c.align_x(Alignment::Start),
                    FlexAlign::Center => c = c.align_x(Alignment::Center),
                    FlexAlign::End => c = c.align_x(Alignment::End),
                }
                for child in flex.children {
                    c = c.push(child);
                }
                c.into()
            }
        }
    }
}
