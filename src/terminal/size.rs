use crate::wit::synt::extension::terminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub cols: u16,
    pub rows: u16,
}

impl Size {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self { cols, rows }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

impl From<Size> for terminal::Size {
    fn from(size: Size) -> Self {
        Self {
            cols: size.cols,
            rows: size.rows,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_default_terminal_is_eighty_by_twenty_four() {
        let wire = terminal::Size::from(Size::default());

        assert_eq!((wire.cols, wire.rows), (80, 24));
    }
}
