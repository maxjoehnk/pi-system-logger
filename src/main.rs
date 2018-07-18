extern crate influent;
#[macro_use]
extern crate failure;
extern crate regex;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod clock_frequency;
mod vcgencmd;
mod temperature;
mod voltage;

#[derive(Debug, Deserialize, Serialize)]
struct Configuration {
    interval: i32,
    influx: InfluxConfiguration
}

#[derive(Debug, Deserialize, Serialize)]
struct InfluxConfiguration {
    username: String,
    password: String,
    database: String,
    hosts: Vec<String>
}

fn main() -> Result<(), failure::Error> {
    println!("=== Temperature ===");
    println!("CPU Temperature: {:?}", temperature::get_cpu()?);
    println!("GPU Temperature: {:?}", temperature::get_gpu()?);
    println!("\n==== Frequency ====");
    println!("ARM Clock Frequency:   {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::Arm)?);
    println!("Core Clock Frequency:  {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::Core)?);
    println!("h264 Clock Frequency:  {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::H264)?);
    println!("isp Clock Frequency:   {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::Isp)?);
    println!("v3d Clock Frequency:   {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::V3d)?);
    println!("uart Clock Frequency:  {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::Uart)?);
    println!("pwm Clock Frequency:   {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::Pwm)?);
    println!("emmc Clock Frequency:  {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::Emmc)?);
    println!("pixel Clock Frequency: {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::Pixel)?);
    println!("vec Clock Frequency:   {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::Vec)?);
    println!("hdmi Clock Frequency:  {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::Hdmi)?);
    println!("dpi Clock Frequency:   {:?}", clock_frequency::get_clock(clock_frequency::ClockTypes::Dpi)?);
    println!("\n===== Voltage =====");
    println!("Core Voltage:    {:?}", voltage::get_voltage(voltage::Voltages::Core)?);
    println!("Sdram C Voltage: {:?}", voltage::get_voltage(voltage::Voltages::SdramC)?);
    println!("Sdram I Voltage: {:?}", voltage::get_voltage(voltage::Voltages::SdramI)?);
    println!("Sdram P Voltage: {:?}", voltage::get_voltage(voltage::Voltages::SdramP)?);

    Ok(())
}