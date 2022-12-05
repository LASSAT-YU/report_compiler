use crate::input_reports::{load_input_files, InputFiles};
use crate::settings::Cli;
use std::fs;

pub fn run(args: &Cli) -> anyhow::Result<String> {
    let input = load_input_files(args)?;
    let output = generate_output(&input, args)?;
    Ok(output)
}

fn generate_output(input: &InputFiles, args: &Cli) -> anyhow::Result<String> {
    todo!()
}
pub fn save_output(output: &str, args: &Cli) -> anyhow::Result<String> {
    let file_name = args.output.clone();
    fs::write(&file_name, output)?;
    Ok(file_name)
}
