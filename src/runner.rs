use crate::input_reports::{load_input_files, InputFiles};
use crate::settings::Cli;

pub fn run(args: &Cli) -> anyhow::Result<String> {
    let input = load_input_files(args)?;
    let output = generate_output(&input, args)?;
    Ok(output)
}

fn generate_output(input: &InputFiles, args: &Cli) -> anyhow::Result<String> {
    todo!()
}
pub fn save_output(output: &str, args: &Cli) -> anyhow::Result<String> {
    todo!()
}
