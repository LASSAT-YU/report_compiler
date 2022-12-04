use chrono::NaiveDate;
use clap::Parser;
use std::error::Error;
use std::fs;

pub fn run(args: &Cli) -> Result<String, Box<dyn Error>> {
    todo!();
    // TODO Load input files into memory

    // TODO Generate output from input files

    // TODO Save output to disk and report where file was saved
    fs::read_dir("./change_this_path").unwrap();
    Ok("".to_string())
}

pub fn save_output(output: &str, args: &Cli) -> Result<String, Box<dyn Error>> {
    todo!()
}

#[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(help = "Earliest date to include")]
    start_date: NaiveDate,

    #[arg(help = "Last date to include (used as date of report)")]
    end_date: NaiveDate,

    #[arg(
        short = 'f',
        default_value = "reports",
        help = "Path to top level folder that team folders are in"
    )]
    folder: String,

    #[arg(
        short = 'n',
        default_value = "LASSAT Bi-Weekly Report",
        help = "Heading for report"
    )]
    heading: String,

    #[arg(
        short = 't',
        default_value = "40",
        help = "Max length to allow for task name"
    )]
    max_task_name: u8,
}

struct InputFiles {
    // TODO Ignore files that start with .
    teams: Vec<TeamFiles>,
}

struct TeamFiles {
    files: Vec<InputFile>,
}

struct InputFile {
    date: NaiveDate,
    member_name: String,
    is_team_lead: bool,
    summary: String,
    cancelled: Vec<Task>,
    planned: Vec<Task>,
    in_progress: Vec<Task>,
    complete: Vec<Task>,
}

struct Task {
    name: String,
    comment: String, // TODO trim spaces
}
