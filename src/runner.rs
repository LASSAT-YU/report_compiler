use std::fmt::Write;
use std::fs;

use crate::input_reports::{load_input_files, AllInputFiles};
use crate::settings::Cli;
use crate::utils::StringUtils;

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

fn generate_output(input: &AllInputFiles, args: &Cli) -> anyhow::Result<String> {
    // Code written with a preference on readability over speed
    // (like adding eol in separate function, so each doesn't require a comment)
    // unwrap on writeln should never panic unless the system runs out of memory as
    // errors come from inability to write to underlying store and in this case that is
    // memory
    let mut result = format!("# {}\n", &args.heading);
    result.add_eol();

    // Add date of report (Uses last day included in the report)
    writeln!(result, "{}", args.end_date.format("*%F*")).unwrap();
    result.add_eol();

    // Table of Contents
    result.push_str("## Table of Contents\n");
    result.add_eol();
    for (ind_team, team) in input.iter().enumerate() {
        let ind_team = ind_team + 1;
        writeln!(result, "{ind_team}. {}", team.name).unwrap();

        result.add_indent(1);
        writeln!(result, "1. Summaries").unwrap();
        for (ind_member, team_member) in team.files_by_member().iter().enumerate() {
            let ind_member = ind_member + 1;
            result.add_indent(2);
            writeln!(result, "{ind_member}. {}", team_member.display_name()).unwrap();
        }
        result.add_indent(1);
        writeln!(result, "2. Tasks").unwrap();
    }

    Ok(result)
}
