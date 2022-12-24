use clap::{ Args, Parser, Subcommand };
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Arguments {
    #[clap(subcommand)]
    mode: Mode,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Mode {
    /// Set brightness % (0-100)
    Set(SetCommand),

    /// Get current brightness %
    Get,

    /// Increase brightness by n%
    Increase(IncreaseCommand),

    /// Decrease brightness by n%
    Decrease(DecreaseCommand),
}

#[derive(Debug, Args)]
struct SetCommand {
    /// The desired percentage of brightness (0-100)
    brightness: u8
}

#[derive(Debug, Args)]
struct IncreaseCommand {
    /// The desired percentage of brightness (0-100) to increase by
    brightness: u8
}

#[derive(Debug, Args)]
struct DecreaseCommand {
    /// The desired percentage of brightness (0-100) to decrease by
    brightness: u8
}

fn read_raw_brightness() -> u32 {
    let path = "/sys/class/backlight/intel_backlight/brightness";

    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read from {}", path),
    };
    let mut buffer = BufReader::new(file);

    let mut first_line = String::new();

    buffer.read_line(&mut first_line).expect("Unable to read /sys/class/backlight/intel_backlight/brightness");

    first_line.trim_end().parse::<u32>().unwrap()
}

fn read_raw_max_brightness() -> u32 {
    let path = "/sys/class/backlight/intel_backlight/max_brightness";

    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read from {}", path),
    };
    let mut buffer = BufReader::new(file);

    let mut first_line = String::new();

    buffer.read_line(&mut first_line).expect("Unable to read /sys/class/backlight/intel_backlight/max_brightness");

    first_line.trim_end().parse::<u32>().unwrap()
}

fn get_brightness() -> u8 {
    let current_brightness: f32 = (read_raw_brightness() as f32) / (read_raw_max_brightness() as f32) * 100.0;
    current_brightness as u8
}

fn set_brightness(brightness: u8) -> Result<(), Box<dyn std::error::Error>> { 
    let mut cleaned_brightness: u32 = brightness as u32;
    if cleaned_brightness > 100 { cleaned_brightness = 100 };
    // u32 min. is 0, so can't be < 100, brightness % now clean

    let desired_brightness: u32 = ((read_raw_max_brightness() * cleaned_brightness) as f32 * 0.01) as u32;
    fs::write("/sys/class/backlight/intel_backlight/brightness", desired_brightness.to_string())?;
    Ok(())
}

fn increase_brightness(increase_by: u8) -> Result<(), Box<dyn std::error::Error>> { 
    set_brightness(get_brightness()+increase_by)?;
    Ok(())
}
fn decrease_brightness(decrease_by: u8) -> Result<(), Box<dyn std::error::Error>> {
    set_brightness(get_brightness()-decrease_by)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();
    
    use crate::Mode::*;
    match args.mode {
        Set(set_command) => {set_brightness(set_command.brightness)?;},
        Get => {println!("{}", get_brightness());},
        Increase(increase_command) => {increase_brightness(increase_command.brightness)?;},
        Decrease(decrease_command) => {decrease_brightness(decrease_command.brightness)?;},
    }
    Ok(())
}
