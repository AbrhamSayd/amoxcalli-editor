#[derive(Default, PartialEq, Clone, Copy)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
}

impl Mode {
    pub fn is_normal(&self) -> bool {
        matches!(self, Mode::Normal)
    }

    pub fn is_insert(&self) -> bool {
        matches!(self, Mode::Insert)
    }
}