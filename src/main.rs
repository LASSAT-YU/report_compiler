use std::error::Error;

use clap::Parser;

use report_compiler::{run, save_output, Cli};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let output = run(&args)?;
    let filename = save_output(&output, &args)?;
    println!("Saved output to {filename}");
    Ok(())
}
