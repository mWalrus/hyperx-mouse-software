pub struct Command {
    pub data: Vec<u8>,
}

impl Command {
    pub fn set_color(color: &[u8; 3]) -> Self {
        let data: Vec<u8> = vec![
            0xd2, 0x00, 0x00, 0x08, color[0], color[1], color[2], color[0], color[1], color[2],
            0x64,
        ];
        Self { data }.pad()
    }

    // informs device to persist all changes
    pub fn persist() -> Self {
        Self {
            data: vec![0xde, 0xff],
        }
        .pad()
    }

    pub fn set_dpi_profile_dpi(id: u8, dpi: u8) -> Self {
        Self {
            data: vec![0xd3, 0x02, id, 0x02, dpi],
        }
        .pad()
    }

    pub fn set_dpi_profile_color(id: u8, color: [u8; 3]) -> Self {
        Self {
            data: vec![0xd3, 0x03, id, 0x03, color[0], color[1], color[2]],
        }
        .pad()
    }

    pub fn __unknown_command_1() -> Self {
        Self {
            data: vec![
                0xd2, 0x00, 0x00, 0x08, 0x9b, 0x00, 0xdc, 0x9b, 0x00, 0xdc, 0x64,
            ],
        }
        .pad()
    }

    fn pad(mut self) -> Self {
        let len = self.data.len();
        let required_padding = (8 - (len % 8)) % 8;

        for _ in 0..required_padding {
            self.data.push(0x00);
        }
        self
    }
}
