use crate::input_reports::load_input_files;
use crate::settings::Cli;
use std::error::Error;

pub fn run(args: &Cli) -> Result<String, Box<dyn Error>> {
    let input = load_input_files(args)?;
    let output = generate_output(&input, args)?;
    Ok(output)
}

fn generate_output(input: &str, args: &Cli) -> Result<String, Box<dyn Error>> {
    todo!()
}
pub fn save_output(output: &str, args: &Cli) -> Result<String, Box<dyn Error>> {
    todo!()
}
