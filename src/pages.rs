#[derive(PartialEq, Eq)]
#[repr(usize)]
pub enum Pages {
    StatusPAGE,
    InfoPAGE,
    ConfigPage,
}

impl Pages {
    pub fn to_index(&self) -> usize {
        match self {
            Pages::StatusPAGE => 0,
            Pages::InfoPAGE => 1,
            Pages::ConfigPage => 2,
        }
    }

    pub fn change_page(&mut self, value: u32) -> Pages {
        match value {
            0 => Pages::StatusPAGE,
            1 => Pages::InfoPAGE,
            2 => Pages::ConfigPage,
            _ => Pages::StatusPAGE,
        }
    }
}
