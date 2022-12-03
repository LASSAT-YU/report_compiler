use chrono::NaiveDate;
use clap::Parser;

#[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 's', help = "Earliest date to include")]
    start_date: NaiveDate,

    #[arg(short = 'e', help = "Last date to include")]
    end_date: NaiveDate,

    #[arg(
        short = 'f',
        default_value = "reports",
        help = "Path to top level folder that team folders are in"
    )]
    folder: Option<String>,
}
