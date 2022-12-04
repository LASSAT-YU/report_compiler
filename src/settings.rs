use chrono::NaiveDate;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(help = "Earliest date to include")]
    pub start_date: NaiveDate,

    #[arg(help = "Last date to include (used as date of report)")]
    pub end_date: NaiveDate,

    #[arg(
        short = 'f',
        default_value = "reports",
        help = "Path to top level folder that team folders are in"
    )]
    pub folder: String,

    #[arg(
        short = 'n',
        default_value = "LASSAT Bi-Weekly Report",
        help = "Heading for report"
    )]
    pub heading: String,

    #[arg(
        short = 't',
        default_value = "40",
        help = "Max length to allow for task name"
    )]
    pub max_task_name: u8,
}
