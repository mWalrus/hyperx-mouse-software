use std::time::Duration;

use crate::{command::Command, polling_rate::PollingRate};
use rusb::{Device, DeviceDescriptor, DeviceHandle, UsbContext};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MouseError {
    #[error("Failed to detach kernel driver: {0:?}")]
    DetachKernelDriver(rusb::Error),
    #[error("Failed to attach kernel driver: {0:?}")]
    AttachKernelDriver(rusb::Error),
    #[error("Failed to claim the device interface: {0:?}")]
    ClaimInterface(rusb::Error),
    #[error("Failed to release the device interface: {0:?}")]
    ReleaseInterface(rusb::Error),
    #[error("Failed to set alternative active setting for device interface: {0:?}")]
    AlternateSetting(rusb::Error),
    #[error("Failed to write to device: {0:?}")]
    WriteError(rusb::Error),
    #[error("The specified dpi profile is invalid")]
    InvalidDPIProfile,
}

type MouseResult<T> = Result<T, MouseError>;

pub enum MouseAction {
    SetColor([u8; 3], u8),
    SetDPIProfileDPI(u8, u8),
    SetDPIProfileColor(u8, [u8; 3]),
    SetPollingRate(PollingRate),
    SetLowPowerWarn(u8),
    Persist,
}

pub struct Mouse<C: UsbContext> {
    pub dev: Device<C>,
    pub descriptor: DeviceDescriptor,
    pub handle: DeviceHandle<C>,
    pub iface: u8,
    pub dpi_profiles: [DPIProfile; 4],
    pub polling_rate: PollingRate,
    pub power_warn_at: u8,
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
            polling_rate: PollingRate::Hz1000,
            power_warn_at: 10, // percentage
        }
    }

    fn detach(&mut self) -> MouseResult<()> {
        self.handle
            .detach_kernel_driver(self.iface)
            .map_err(MouseError::DetachKernelDriver)?;
        self.handle
            .claim_interface(self.iface)
            .map_err(MouseError::ClaimInterface)?;
        self.handle
            .set_alternate_setting(self.iface, 0)
            .map_err(MouseError::AlternateSetting)?;
        Ok(())
    }

    fn release(&mut self) -> MouseResult<()> {
        self.handle
            .release_interface(self.iface)
            .map_err(MouseError::ReleaseInterface)?;
        self.handle
            .attach_kernel_driver(self.iface)
            .map_err(MouseError::AttachKernelDriver)?;
        Ok(())
    }

    fn write(&mut self, cmd: Command) -> MouseResult<()> {
        println!("Writing data: {:02x?}", &cmd.data);
        let byte_count = self
            .handle
            .write_interrupt(self.iface, &cmd.data, Duration::from_millis(1000))
            .map_err(MouseError::WriteError)?;
        println!("Wrote {byte_count} bytes");
        Ok(())
    }

    fn set_color(&mut self, color: [u8; 3], opacity: u8) -> MouseResult<()> {
        self.write(Command::set_color(color, opacity))?;
        Ok(())
    }

    fn validate_dpi_profile_id(&self, id: u8) -> MouseResult<usize> {
        let i = Into::<usize>::into(id);
        if i >= self.dpi_profiles.len() {
            return Err(MouseError::InvalidDPIProfile);
        }
        Ok(i)
    }

    fn set_dpi_for_profile(&mut self, id: u8, dpi: u8) -> MouseResult<()> {
        let i = self.validate_dpi_profile_id(id)?;
        self.write(Command::set_dpi_profile_dpi(id, dpi))?;
        self.dpi_profiles[i].dpi = dpi;
        Ok(())
    }

    fn set_color_for_profile(&mut self, id: u8, color: [u8; 3]) -> MouseResult<()> {
        let i = self.validate_dpi_profile_id(id)?;
        self.write(Command::set_dpi_profile_color(id, color))?;
        self.dpi_profiles[i].color = color;
        Ok(())
    }

    fn set_polling_rate(&mut self, rate: PollingRate) -> MouseResult<()> {
        self.write(Command::set_polling_rate(rate as u8))?;
        self.polling_rate = rate;
        Ok(())
    }

    fn set_low_power_warn(&mut self, mut percentage: u8) -> MouseResult<()> {
        percentage = percentage.min(25).max(1);
        self.write(Command::set_low_power_warn(percentage))?;
        self.power_warn_at = percentage;
        Ok(())
    }

    fn persist(&mut self) -> MouseResult<()> {
        self.write(Command::persist())?;
        Ok(())
    }

    pub fn perform_action(&mut self, action: MouseAction) -> MouseResult<()> {
        self.detach()?;
        match action {
            MouseAction::SetColor(c, o) => self.set_color(c, o),
            MouseAction::Persist => self.persist(),
            MouseAction::SetDPIProfileDPI(id, dpi) => self.set_dpi_for_profile(id, dpi),
            MouseAction::SetDPIProfileColor(id, color) => self.set_color_for_profile(id, color),
            MouseAction::SetPollingRate(rate) => self.set_polling_rate(rate),
            MouseAction::SetLowPowerWarn(bat_percentage) => self.set_low_power_warn(bat_percentage),
            _ => Ok(()),
        }?;
        self.release()?;
        Ok(())
    }
}
