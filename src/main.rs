use clap::{ Args, Parser, Subcommand };

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Arguments
{
    #[clap(subcommand)]
    mode: Mode,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Mode
{
    /// Set Brightness
    Set(SetCommand),

    /// Get current brightness
    Get,

    /// Increase Brightness by
    Increase(IncreaseCommand),
    Inc(IncreaseCommand),

    /// Decrease Brightness By
    Decrease(DecreaseCommand),
    Dec(DecreaseCommand),
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

fn get_brightness() -> u8 { 0 }
fn set_brightness(brightness: u8) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
fn increase_brightness(increase_by: u8) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
fn decrease_brightness(decrease_by: u8) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }

fn main() {
    let args = Arguments::parse();
    
    use crate::Mode::*;
    match args.mode {
        Set(set_command) => {},
        Get => {},
        Increase(increase_command) | Inc(increase_command) => {},
        Decrease(decrease_command) | Dec(decrease_command) => {},
    }
}
