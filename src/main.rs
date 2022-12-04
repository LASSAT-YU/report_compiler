use clap::Parser;
use report_compiler::runner::{run, save_output};
use report_compiler::settings::Cli;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let output = run(&args)?;
    let filename = save_output(&output, &args)?;
    println!("Saved output to {filename}");
    Ok(())
}
