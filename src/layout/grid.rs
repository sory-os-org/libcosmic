//! Grid layout — grille moderne avec colonnes flexibles.
//!
//! Supporte le spanning de colonnes, le gap uniforme, et
//! l'alignement des items dans la grille.

use crate::widget::{column, row};
use crate::Element;

/// Span de colonne pour un item de grille.
#[derive(Debug, Clone, Copy)]
pub struct GridSpan(pub usize);

/// Grille moderne avec colonnes flexibles.
pub struct Grid<'a, Message> {
    columns: usize,
    gap: f32,
    children: Vec<(Element<'a, Message>, Option<GridSpan>)>,
}

impl<'a, Message> Grid<'a, Message> {
    pub fn new(columns: usize) -> Self {
        Self {
            columns,
            gap: 0.0,
            children: Vec::new(),
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push((child.into(), None));
        self
    }

    pub fn push_span(mut self, child: impl Into<Element<'a, Message>>, span: GridSpan) -> Self {
        self.children.push((child.into(), Some(span)));
        self
    }
}

impl<'a, Message: 'a> From<Grid<'a, Message>> for Element<'a, Message> {
    fn from(grid: Grid<'a, Message>) -> Self {
        let cols = grid.columns.max(1);
        let gap = grid.gap as u16;

        let mut rows: Vec<Element<'a, Message>> = Vec::new();
        let mut current_row: Vec<Element<'a, Message>> = Vec::new();
        let mut col_idx = 0;

        for (child, span) in grid.children {
            let span = span.map_or(1, |s| s.0.min(cols));
            if col_idx + span > cols {
                flush_row(&mut rows, &mut current_row, gap);
                col_idx = 0;
            }
            current_row.push(child);
            col_idx += span;
            if col_idx >= cols {
                flush_row(&mut rows, &mut current_row, gap);
                col_idx = 0;
            }
        }

        if !current_row.is_empty() {
            flush_row(&mut rows, &mut current_row, gap);
        }

        let mut col = column::with_capacity(rows.len());
        if gap > 0 {
            col = col.spacing(gap);
        }
        for row_elem in rows {
            col = col.push(row_elem);
        }
        col.into()
    }
}

fn flush_row<'a, Message: 'a>(
    rows: &mut Vec<Element<'a, Message>>,
    current: &mut Vec<Element<'a, Message>>,
    gap: u16,
) {
    if current.is_empty() {
        return;
    }
    let mut r = row::with_capacity(current.len());
    if gap > 0 {
        r = r.spacing(gap);
    }
    for child in current.drain(..) {
        r = r.push(child);
    }
    rows.push(r.into());
}
