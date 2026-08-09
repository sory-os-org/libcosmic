//! Table de données moderne SoryOS — pattern shadcn Table.

use crate::iced::{Alignment, Length, Padding};
use crate::widget::{column, container, row, text};
use crate::Element;
use std::borrow::Cow;

/// Une ligne de table avec des cellules (chaque cellule est un string pour simplicité).
pub struct TableRow<'a> {
    cells: Vec<Cow<'a, str>>,
}

impl<'a> TableRow<'a> {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn push_cell(mut self, cell: impl Into<Cow<'a, str>>) -> Self {
        self.cells.push(cell.into());
        self
    }
}

/// Colonne de table (header only).
pub struct TableColumn {
    header: Cow<'static, str>,
    width: Length,
}

impl TableColumn {
    pub fn new(header: impl Into<Cow<'static, str>>) -> Self {
        Self {
            header: header.into(),
            width: Length::Fill,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

/// Table moderne avec colonnes et lignes.
pub struct ModernTable<'a, Message> {
    columns: Vec<TableColumn>,
    rows: Vec<TableRow<'a>>,
    on_row_click: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message: Clone + 'static> ModernTable<'a, Message> {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            on_row_click: None,
        }
    }

    pub fn column(mut self, col: TableColumn) -> Self {
        self.columns.push(col);
        self
    }

    pub fn columns(mut self, cols: impl IntoIterator<Item = TableColumn>) -> Self {
        self.columns.extend(cols);
        self
    }

    pub fn row(mut self, row: TableRow<'a>) -> Self {
        self.rows.push(row);
        self
    }

    pub fn rows(mut self, rows: impl IntoIterator<Item = TableRow<'a>>) -> Self {
        self.rows.extend(rows);
        self
    }

    pub fn on_row_click(mut self, f: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_row_click = Some(Box::new(f));
        self
    }
}

impl<'a, Message: Clone + 'static> From<ModernTable<'a, Message>> for Element<'a, Message> {
    fn from(table: ModernTable<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        if table.columns.is_empty() {
            return text::body("Table vide").into();
        }

        // Extract column headers before consuming columns
        let headers: Vec<(String, Length)> = table.columns.iter()
            .map(|c| (c.header.clone().into_owned(), c.width))
            .collect();
        let num_cols = headers.len();

        // Header row
        let mut header_row = row::with_capacity(num_cols)
            .spacing(spacing.space_m)
            .align_y(Alignment::Center);

        for (header_text, width) in &headers {
            header_row = header_row.push(
                text::caption(header_text.clone()).width(*width),
            );
        }

        let mut content = column::with_capacity(table.rows.len() + 1)
            .spacing(0)
            .push(
                container(header_row)
                    .padding(Padding::from([spacing.space_s, spacing.space_m]))
                    .width(Length::Fill)
                    .class(crate::theme::Container::Card),
            );

        // Data rows
        for (row_idx, table_row) in table.rows.into_iter().enumerate() {
            let mut data_row = row::with_capacity(num_cols)
                .spacing(spacing.space_m)
                .align_y(Alignment::Center);

            for (col_idx, (_, width)) in headers.iter().enumerate() {
                let cell_text = if col_idx < table_row.cells.len() {
                    table_row.cells[col_idx].clone().into_owned()
                } else {
                    String::new()
                };
                data_row = data_row.push(
                    container(text::body(cell_text)).width(*width),
                );
            }

            let row_container = container(data_row)
                .padding(Padding::from([spacing.space_s, spacing.space_m]))
                .width(Length::Fill);

            let row_element: Element<'_, Message> = if let Some(on_row_click) = &table.on_row_click {
                let idx = row_idx;
                let msg = on_row_click(idx);
                crate::widget::mouse_area(row_container).on_press(msg).into()
            } else {
                row_container.into()
            };

            content = content.push(row_element);
        }

        content.into()
    }
}
