use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Strip {
    frames_per_second: u8,
    name: String,
    row: u8,
}

impl Strip {
    pub fn frames_per_second(&self) -> u8 {
        self.frames_per_second
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn row(&self) -> u8 {
        self.row
    }
}
