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
    // Code written with a preference for readability over speed
    // (like adding eol in separate function, so each doesn't require a comment)
    // unwrap on writeln should never panic unless the system runs out of memory as
    // errors come from inability to write to underlying store and in this case that is
    // memory
    let mut result = format!("# {}\n", &args.heading);
    result.add_eol();

    // Add date of report (Uses last day included in the report)
    writeln!(result, "{}", args.end_date.format("*%F*")).unwrap();
    result.add_eol();

    output_add_toc(input, &mut result);
    result.add_eol();
    output_add_details(input, &mut result);

    Ok(result)
}

fn output_add_details(input: &AllInputFiles, s: &mut String) {
    // Report Details
    writeln!(s, "## Team Reports").unwrap();
    s.add_eol();
    for (ind_team, team) in input.iter().enumerate() {
        let ind_team = ind_team + 1;
        writeln!(s, "### {ind_team}. {}", team.name).unwrap();

        if !team.files.is_empty() {
            s.add_eol();
            writeln!(s, "#### 1. Summaries").unwrap();
            for (ind_member, team_member) in team.files_by_member().iter().enumerate() {
                let ind_member = ind_member + 1;

                s.add_eol();
                writeln!(s, "##### {ind_member}. {}", team_member.display_name()).unwrap();
                s.add_eol();
                {
                    // Insert summaries
                    let mut at_least_one_added = false;
                    let summaries: Vec<&String> =
                        team_member.iter().map(|file| &file.summary).collect();
                    for summary in summaries {
                        if !summary.is_empty() {
                            if at_least_one_added {
                                // Prefix with a <hr />
                                s.add_eol_max_2();
                                writeln!(s, "---").unwrap();
                                s.add_eol();
                            }
                            write!(s, "{summary}").unwrap();
                            s.add_eol_max_2();
                            at_least_one_added = true;
                        }
                    }
                }
            }
            s.add_eol_max_2();
            writeln!(s, "#### 2. Tasks").unwrap();
            s.add_eol();
            writeln!(s, "##### 1. Summary").unwrap();
            writeln!(s, "\nTODO\n").unwrap(); // TODO Implement table
            for (ind_task, task) in team.tasks().iter().enumerate() {
                let ind_task = ind_task + 2; // First is Summary
                s.add_eol_max_2();
                writeln!(s, "##### {ind_task}. {}", task.display_name()).unwrap();
                if !task.comment.is_empty() {
                    s.add_eol();
                    writeln!(s, "{}", &task.comment).unwrap();
                }
                s.add_eol_max_2();
            }
        } else {
            s.add_eol_max_2();
        }
    }
}

fn output_add_toc(input: &AllInputFiles, s: &mut String) {
    // Table of Contents
    writeln!(s, "## Table of Contents").unwrap();
    s.add_eol();
    for (ind_team, team) in input.iter().enumerate() {
        let ind_team = ind_team + 1;
        writeln!(s, "{ind_team}. {}", team.name).unwrap();

        if !team.files.is_empty() {
            s.add_indent(1);
            writeln!(s, "1. Summaries").unwrap();
            for (ind_member, team_member) in team.files_by_member().iter().enumerate() {
                let ind_member = ind_member + 1;
                s.add_indent(2);
                writeln!(s, "{ind_member}. {}", team_member.display_name()).unwrap();
            }
            s.add_indent(1);
            writeln!(s, "2. Tasks").unwrap();
            s.add_indent(2);
            writeln!(s, "1. Summary").unwrap();
            for (ind_task, task) in team.tasks().iter().enumerate() {
                let ind_task = ind_task + 2; // First is Summary
                s.add_indent(2);
                writeln!(s, "{ind_task}. {}", task.display_name()).unwrap();
            }
        }
    }
}
