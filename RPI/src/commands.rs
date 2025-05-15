#[derive(Debug)]
pub enum Status {
    On,
    Off,
}

impl Status {
    pub fn bool_to_status(status: bool) -> Self {
        match status {
            true => Status::On,
            false => Status::Off,
        }
    }

    pub fn status_to_bool(&self) -> bool {
        match self {
            Self::On => true,
            Self::Off => false,
        }
    }

    pub fn stringify(&self) -> &'static str {
        match self {
            Self::On => "On",
            Self::Off => "Off",
        }
    }
}
