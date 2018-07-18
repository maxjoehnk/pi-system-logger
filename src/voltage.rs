use failure::{Error, err_msg};
use vcgencmd;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Voltage(f32);

#[derive(Display, AsRefStr, Debug)]
pub enum Voltages {
    #[strum(serialize="core")]
    Core,
    #[strum(serialize="sdram_c")]
    SdramC,
    #[strum(serialize="sdram_i")]
    SdramI,
    #[strum(serialize="sdram_p")]
    SdramP
}

pub fn get_voltage(voltage: Voltages) -> Result<Voltage, Error> {
    let re = Regex::new(r"volt=(?P<voltage>[0-9].[0-9]+)V")?;
    let measure_volts_output = vcgencmd::execute(&["measure_volts", voltage.as_ref()])?;
    let caps = re.captures(&measure_volts_output).ok_or(format_err!("Invalid vcgencmd Output: {}", measure_volts_output))?;
    let voltage = &caps["voltage"];
    let voltage = f32::from_str(voltage)?;

    Ok(Voltage(voltage))
}