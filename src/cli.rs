use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  /// The name of the input file or folder
  /// If this is a folder, all files in the folder will be processed
  #[arg(short, long)]
  pub file: String,

  /// Which column to use
  #[arg(short, long, default_value_t = 1)]
  pub column: u8,
  
  /// Skip the first `n` lines
  #[arg(short, long, default_value_t = 1)]
  pub skip: u8
}