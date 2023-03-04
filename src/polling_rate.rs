use std::fmt::Display;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PollingRate {
    Hz125,
    Hz250,
    Hz500,
    Hz1000,
}

impl Display for PollingRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rate = match self {
            PollingRate::Hz125 => "125Hz",
            PollingRate::Hz250 => "250Hz",
            PollingRate::Hz500 => "500Hz",
            PollingRate::Hz1000 => "1000Hz",
        };
        write!(f, "{rate}")
    }
}
