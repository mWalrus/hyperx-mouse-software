use std::time::Duration;

use crate::command::Command;
use rusb::{Device, DeviceDescriptor, DeviceHandle, Result as RusbResult, UsbContext};

pub enum MouseAction {
    SetColor([u8; 3]),
    SetDPIProfileDPI(u8, u8),
    SetDPIProfileColor(u8, [u8; 3]),
    Persist,
}

pub struct Mouse<C: UsbContext> {
    pub dev: Device<C>,
    pub descriptor: DeviceDescriptor,
    pub handle: DeviceHandle<C>,
    pub iface: u8,
    pub dpi_profiles: [DPIProfile; 4],
}

#[derive(Clone, Copy)]
pub struct DPIProfile {
    color: [u8; 3],
    dpi: u8,
}

impl<C: UsbContext> Mouse<C> {
    pub fn new(
        dev: Device<C>,
        descriptor: DeviceDescriptor,
        handle: DeviceHandle<C>,
        iface: u8,
    ) -> Self {
        Self {
            dev,
            descriptor,
            handle,
            iface,
            // FIXME: temporary
            dpi_profiles: [
                DPIProfile {
                    color: [0xff, 0x00, 0x00], // red
                    dpi: 0x04,
                },
                DPIProfile {
                    color: [0x00, 0xd0, 0xff], // cyan
                    dpi: 0x08,
                },
                DPIProfile {
                    color: [0xff, 0xff, 0x00], // yellow
                    dpi: 0x10,
                },
                DPIProfile {
                    color: [0x00, 0xff, 0x00], // green
                    dpi: 0x20,
                },
            ],
        }
    }

    fn detach(&mut self) -> RusbResult<()> {
        self.handle.detach_kernel_driver(self.iface)?;
        self.handle.claim_interface(self.iface)?;
        self.handle.set_alternate_setting(self.iface, 0)?;
        Ok(())
    }

    fn release(&mut self) -> RusbResult<()> {
        self.handle.release_interface(self.iface)?;
        self.handle.attach_kernel_driver(self.iface)?;
        Ok(())
    }

    fn write(&mut self, cmd: Command) -> RusbResult<()> {
        println!("Writing data: {:02x?}", &cmd.data);
        let byte_count =
            self.handle
                .write_interrupt(self.iface, &cmd.data, Duration::from_millis(1000))?;
        println!("Wrote {byte_count} bytes");
        Ok(())
    }

    fn set_color(&mut self, color: [u8; 3]) -> RusbResult<()> {
        self.write(Command::set_color(&color))?;
        Ok(())
    }

    fn is_valid_profile_id(id: u8) -> Result<(), &'static str> {
        if id >= 4 {
            return Err("Invalid Profile ID");
        }
        Ok(())
    }

    fn set_dpi_for_profile(&mut self, id: u8, dpi: u8) -> RusbResult<()> {
        Self::is_valid_profile_id(id).unwrap();
        let i = Into::<usize>::into(id);
        self.write(Command::set_dpi_profile_dpi(id, dpi))?;
        self.dpi_profiles[i].dpi = dpi;
        Ok(())
    }

    fn set_color_for_profile(&mut self, id: u8, color: [u8; 3]) -> RusbResult<()> {
        Self::is_valid_profile_id(id).unwrap();
        let i = Into::<usize>::into(id);
        self.write(Command::set_dpi_profile_color(id, color))?;
        self.dpi_profiles[i].color = color;
        Ok(())
    }

    fn persist(&mut self) -> RusbResult<()> {
        self.write(Command::persist())?;
        Ok(())
    }

    pub fn perform_action(&mut self, action: MouseAction) -> RusbResult<()> {
        self.detach()?;
        match action {
            MouseAction::SetColor(c) => self.set_color(c),
            MouseAction::Persist => self.persist(),
            MouseAction::SetDPIProfileDPI(id, dpi) => self.set_dpi_for_profile(id, dpi),
            MouseAction::SetDPIProfileColor(id, color) => self.set_color_for_profile(id, color),
            _ => Ok(()),
        }?;
        self.release()?;
        Ok(())
    }
}
