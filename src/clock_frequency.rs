use failure::{Error, err_msg};
use vcgencmd;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct ClockFrequency(i32);

#[derive(Display, AsRefStr, Debug)]
pub enum ClockTypes {
    #[strum(serialize="arm")]
    Arm,
    #[strum(serialize="core")]
    Core,
    #[strum(serialize="h264")]
    H264,
    #[strum(serialize="isp")]
    Isp,
    #[strum(serialize="v3d")]
    V3d,
    #[strum(serialize="uart")]
    Uart,
    #[strum(serialize="pwm")]
    Pwm,
    #[strum(serialize="emmc")]
    Emmc,
    #[strum(serialize="pixel")]
    Pixel,
    #[strum(serialize="vec")]
    Vec,
    #[strum(serialize="hdmi")]
    Hdmi,
    #[strum(serialize="dpi")]
    Dpi
}

pub fn get_clock(clock_type: ClockTypes) -> Result<ClockFrequency, Error> {
    let re = Regex::new(r"frequency\(([0-9]+)\)=(?P<clock>[0-9]+)")?;
    let measure_clock_output = vcgencmd::execute(&["measure_clock", clock_type.as_ref()])?;
    let caps = re.captures(&measure_clock_output).ok_or(err_msg("Invalid vcgencmd Output"))?;
    let clock = &caps["clock"];
    let clock = i32::from_str(clock)?;

    Ok(ClockFrequency(clock))
}