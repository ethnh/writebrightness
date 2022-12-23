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

    /// Get Brightness
    Get,

    /// Increase Brightness by
    Inc(IncCommand),

    /// Decrease Brightness By
    Dec(DecCommand)
}

#[derive(Debug, Args)]
struct SetCommand {
    /// The desired percentage of brightness (0-100)
    brightness: u8
}

#[derive(Debug, Args)]
struct IncCommand {
    /// The desired percentage of brightness (0-100) to increase by
    brightness: u8
}

#[derive(Debug, Args)]
struct DecCommand {
    /// The desired percentage of brightness (0-100) to decrease by
    brightness: u8
}

fn main() {
    let args = Arguments::parse();
}
