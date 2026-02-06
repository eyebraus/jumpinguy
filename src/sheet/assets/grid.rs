use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub(in crate::sheet) struct Grid {
    columns: u8,
    rows: u8,
}

impl Grid {
    pub fn columns(&self) -> u8 {
        self.columns
    }

    pub fn rows(&self) -> u8 {
        self.rows
    }
}
