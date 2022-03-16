use std::path::Path;

use color_eyre::Result;
use rusb::open_device_with_vid_pid;
use zkraken_lib::{NZXTDevice, PID, VID};

fn main() -> Result<()> {
    // We need to use RUSB as well because HIDAPI doesn't support the writing to BULK endpoint.
    let mut handle = open_device_with_vid_pid(VID, PID).expect("No Kraken Z device found!");
    let api = hidapi_rusb::HidApi::new()?;
    let hid_device = api.open(VID, PID)?;
    let nzxt_device = NZXTDevice::new(&hid_device, &mut handle, 270)?;

    nzxt_device.set_fan_duty(80)?;
    nzxt_device.set_pump_duty(80)?;

    let image = Path::new("/home/jordyn/Downloads/elmo.gif");
    nzxt_device.set_image(image, 1, true)?;

    Ok(())
}
