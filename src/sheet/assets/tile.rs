use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub(in crate::sheet) struct Tile {
    height: u8,
    width: u8,
}

impl Tile {
    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn width(&self) -> u8 {
        self.width
    }
}
