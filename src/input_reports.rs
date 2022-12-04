use crate::settings::Cli;
use chrono::NaiveDate;
use std::error::Error;
use std::fs;

pub fn load_input_files(args: &Cli) -> Result<String, Box<dyn Error>> {
    for file in fs::read_dir(&args.folder)? {
        println!("{}", file?.path().display());
    }
    todo!()
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
