use std::fs;
use std::path::PathBuf;

use chrono::NaiveDate;

use report_compiler::input_reports::InputFile;
use report_compiler::runner::run;
use report_compiler::settings::Cli;

fn make_settings(start: &str, end: &str, folder: &str) -> Cli {
    let start = NaiveDate::parse_from_str(start, "%F").unwrap();
    let end = NaiveDate::parse_from_str(end, "%F").unwrap();
    Cli {
        start_date: start,
        end_date: end,
        folder: format!("tests/data/{folder}"),
        heading: "LASSAT Bi-Weekly Report".to_string(),
        max_task_name: 40,
    }
}

#[test]
fn empty_template_report() {
    let expected = InputFile {
        date: NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
        member_name: "FirstnameL".to_string(),
        is_team_lead: false,
        summary: Some("".to_string()),
        cancelled: vec![],
        planned: vec![],
        in_progress: vec![],
        complete: vec![],
    };

    let actual = InputFile::load_from_disk(
        &PathBuf::from("tests/data/2022-12-31_FirstnameL.md"),
        &Default::default(),
    )
    .expect("Unable to load template");
    assert_eq!(actual, expected);
}

fn helper(args: Cli, target: &str) {
    let actual = run(&args).unwrap();
    let expected = fs::read_to_string(target).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn example1_all() {
    helper(
        make_settings("2000-01-01", "3000-01-01", "example1"),
        "test1.md",
    );
}

// TODO Add test to skip file in wrong place
