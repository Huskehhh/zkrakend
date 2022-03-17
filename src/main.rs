use std::path::Path;

use color_eyre::Result;
use rusb::open_device_with_vid_pid;
use zkraken_lib::{NZXTDevice, PID, VID};

fn main() -> Result<()> {
    let mut handle = open_device_with_vid_pid(VID, PID).expect("No Kraken Z device found!");
    let nzxt_device = NZXTDevice::new(&mut handle, 270)?;

    let status = nzxt_device.get_status()?;
    let firmware_version = nzxt_device.get_firmware_version()?;

    println!("Status: {:?}", status);
    println!("Firmware version: {}", firmware_version);

    nzxt_device.set_fan_duty(80)?;
    nzxt_device.set_pump_duty(80)?;

    let image = Path::new("/home/jordyn/Downloads/elmo.gif");
    nzxt_device.set_image(image, 1, true)?;

    Ok(())
}
