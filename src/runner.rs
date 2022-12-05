use crate::input_reports::{load_input_files, InputFiles};
use crate::settings::Cli;
use crate::utils::StringUtils;
use std::fs;

pub fn run(args: &Cli) -> anyhow::Result<String> {
    let input = load_input_files(args)?;
    let output = generate_output(&input, args)?;
    Ok(output)
}

pub fn save_output(output: &str, args: &Cli) -> anyhow::Result<String> {
    let file_name = args.output.clone();
    fs::write(&file_name, output)?;
    Ok(file_name)
}

fn generate_output(input: &InputFiles, args: &Cli) -> anyhow::Result<String> {
    // Code written with a preference on readability over speed (like adding eol in separate function, so each doesn't require a comment)
    let mut result = format!("# {}\n", &args.heading);
    result.add_eol();

    // Add date of report (Uses last day included in the report)
    result.push_str(&args.end_date.format("*%F*\n").to_string());
    result.add_eol();

    // Table of Contents
    result.push_str("## Table of Contents\n");
    result.add_eol();

    Ok(result)
}
