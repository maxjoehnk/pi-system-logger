use failure::{err_msg, Error};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use vcgencmd;

#[derive(Debug, Copy, Clone)]
pub struct Temperature(f32);

pub fn get_cpu() -> Result<Temperature, Error> {
    let mut f = File::open("/sys/class/thermal/thermal_zone0/temp")?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let temp = f32::from_str(&contents.trim())?;

    Ok(Temperature(temp / 1000f32))
}

pub fn get_gpu() -> Result<Temperature, Error> {
    let re = Regex::new("temp=(?P<temp>[0-9]+.[0-9])")?;
    let gpu_temp_output = vcgencmd::execute(&["measure_temp"])?;
    let caps = re.captures(&gpu_temp_output).ok_or(err_msg("Invalid vcgencmd Output"))?;
    let temp = &caps["temp"];
    let temp = f32::from_str(temp)?;

    Ok(Temperature(temp))
}