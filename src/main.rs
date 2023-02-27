mod command;
mod mouse;

use mouse::Mouse;
use rusb::{Context, Result, UsbContext};

use crate::mouse::MouseAction;

const ID_VENDOR: u16 = 0x03f0;
const ID_PRODUCT: u16 = 0x048e;

// hard code the color for now
const COLOR: [u8; 3] = [0xC7, 0x00, 0xC6];

fn main() -> Result<()> {
    match Context::new() {
        Ok(mut ctx) => match open_mouse_device(&mut ctx) {
            Some(mut mouse) => {
                println!(
                    "[OPENED DEVICE]: vendor id: {:04x}, product id: {:04x}",
                    mouse.descriptor.vendor_id(),
                    mouse.descriptor.product_id()
                );
                mouse.perform_action(MouseAction::SetColor(COLOR))?;
                // profile 2
                mouse.perform_action(MouseAction::SetDPIProfileDPI(0x01, 0x04))?;
                mouse.perform_action(MouseAction::SetDPIProfileColor(0x01, [0xf4, 0x78, 0x35]))?;

                mouse.perform_action(MouseAction::Persist)?;
            }
            None => println!("Failed to open device {ID_VENDOR:04x}:{ID_PRODUCT:04x}"),
        },
        Err(e) => panic!("Could not initialize libusb: {e}"),
    }

    Ok(())
}

fn open_mouse_device<C: UsbContext>(ctx: &mut C) -> Option<Mouse<C>> {
    let devices = match ctx.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };
    for device in devices.iter() {
        let desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if desc.vendor_id() == ID_VENDOR && desc.product_id() == ID_PRODUCT {
            match device.open() {
                Ok(handle) => {
                    return Some(Mouse::new(device, desc, handle, 2));
                }
                Err(e) => {
                    eprintln!("{e:?}");
                    continue;
                }
            }
        }
    }
    None
}
