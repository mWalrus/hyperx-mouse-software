use crate::command;

#[derive(Debug)]
pub struct Command {
    pub data: Vec<u8>,
}

impl Command {
    pub fn set_color(color: [u8; 3], opacity: u8) -> Self {
        command!(
            0xd2, 0x00, 0x00, 0x08, color[0], color[1], color[2], color[0], color[1], color[2],
            opacity
        )
    }

    pub fn persist() -> Self {
        command!(0xde, 0xff)
    }

    pub fn set_dpi_profile_dpi(id: u8, dpi: u8) -> Self {
        command!(0xd3, 0x02, id, 0x02, dpi)
    }

    pub fn set_dpi_profile_color(id: u8, color: [u8; 3]) -> Self {
        command!(0xd3, 0x03, id, 0x03, color[0], color[1], color[2])
    }

    pub fn set_polling_rate(rate: u8) -> Self {
        command!(0xd0, 0x00, 0x00, 0x01, rate)
    }

    pub fn set_gradient_part(part: u8, colors: [[u8; 3]; 20]) -> Self {
        let mut data = vec![0xda, 0x78, part, 0x3c];
        for color in colors {
            for channel in color {
                data.push(channel);
            }
        }
        Command {
            data: crate::pad!(data),
        }
    }

    pub fn set_low_power_warn(percentage: u8) -> Self {
        command!(0xd1, 0x00, 0x00, 0x01, percentage)
    }
}

#[cfg(test)]
mod tests {
    use super::{command, Command};

    #[test]
    fn should_pad_to_8_bytes() {
        let cmd = command!(0x00);
        assert_eq!(8, cmd.data.len());
    }

    #[test]
    fn should_not_pad() {
        let cmd = command!(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
        assert_eq!(8, cmd.data.len());
    }
}

#[macro_export]
macro_rules! command {
    ($($byte:expr),*) => {
        {
            Command { data: crate::pad!(vec![$($byte),*]) }
        }
    };
}

#[macro_export]
macro_rules! pad {
    ($v:expr) => {{
        let len = $v.len();
        let required_padding = (8 - (len % 8)) % 8;

        for _ in 0..required_padding {
            $v.push(0x00);
        }
        $v
    }};
}
