#[derive(Debug)]
pub struct Command {
    pub data: Vec<u8>,
}

impl Command {
    fn construct(mut data: Vec<u8>) -> Self {
        let len = data.len();
        let required_padding = (8 - (len % 8)) % 8;

        for _ in 0..required_padding {
            data.push(0x00);
        }
        Self { data }
    }

    pub fn set_color(color: [u8; 3], opacity: u8) -> Self {
        Self::construct(vec![
            0xd2, 0x00, 0x00, 0x08, color[0], color[1], color[2], color[0], color[1], color[2],
            opacity,
        ])
    }

    // informs device to persist all changes
    pub fn persist() -> Self {
        Self::construct(vec![0xde, 0xff])
    }

    pub fn set_dpi_profile_dpi(id: u8, dpi: u8) -> Self {
        Self::construct(vec![0xd3, 0x02, id, 0x02, dpi])
    }

    pub fn set_dpi_profile_color(id: u8, color: [u8; 3]) -> Self {
        Self::construct(vec![0xd3, 0x03, id, 0x03, color[0], color[1], color[2]])
    }

    pub fn set_polling_rate(rate: u8) -> Self {
        Self::construct(vec![0xd0, 0x00, 0x00, 0x01, rate])
    }

    pub fn set_gradient_part(part: u8, colors: [[u8; 3]; 20]) -> Self {
        let mut data = vec![0xda, 0x78, part, 0x3c];
        for color in colors {
            for channel in color {
                data.push(channel);
            }
        }
        Self::construct(data)
    }

    pub fn set_low_power_warn(percentage: u8) -> Self {
        Self::construct(vec![0xd1, 0x00, 0x00, 0x01, percentage])
    }
}
