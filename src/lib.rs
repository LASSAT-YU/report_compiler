use chrono::NaiveDate;
use clap::Parser;

pub fn run(args: Cli) -> String {
    todo!()
}

#[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 's', help = "Earliest date to include")]
    start_date: NaiveDate,

    #[arg(short = 'e', help = "Last date to include (used as date of report)")]
    end_date: NaiveDate,

    #[arg(
        short = 'f',
        default_value = "reports",
        help = "Path to top level folder that team folders are in"
    )]
    folder: String,
}

struct InputFiles {
    teams: Vec<TeamFiles>,
}

struct TeamFiles {
    files: Vec<InputFile>,
}

struct InputFile {
    date: NaiveDate,
    member_name: String,
    cancelled: Vec<Task>,
    planned: Vec<Task>,
    in_progress: Vec<Task>,
    complete: Vec<Task>,
}

struct Task {
    name: String,
    comment: String, // TODO trim spaces
}
