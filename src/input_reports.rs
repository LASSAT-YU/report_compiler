use crate::settings::Cli;
use crate::utils::FileNameExtract;
use anyhow::Context;
use chrono::{Datelike, NaiveDate};
use std::fs;
use std::path::PathBuf;

pub fn load_input_files(args: &Cli) -> anyhow::Result<InputFiles> {
    InputFiles::load_from_disk(args)
}

pub struct InputFiles {
    pub teams: Vec<TeamFiles>,
}

impl InputFiles {
    fn load_from_disk(args: &Cli) -> anyhow::Result<Self> {
        let mut result = Self { teams: vec![] };

        for entry in fs::read_dir(&args.folder)
            .with_context(|| format!("Failed to read dir: {}", &args.folder))?
        {
            // Iterating top level folder, expecting team folders only
            let path = entry?.path();
            if path.file_name_to_string_lossy().starts_with('.') {
                println!("Skipping '{}' because it starts with a '.'", path.display());
                continue;
            }
            if path.is_file() {
                eprintln!(
                    "Skipping '{}' because not expecting files at top level only team folders",
                    path.display()
                );
                continue;
            }
            result.teams.push(TeamFiles::load_from_disk(path, args)?);
        }
        Ok(result)
    }
}

pub struct TeamFiles {
    pub name: String,
    pub files: Vec<InputFile>,
}

impl TeamFiles {
    fn load_from_disk(dir_team: PathBuf, args: &Cli) -> anyhow::Result<Self> {
        let mut result = Self {
            name: dir_team.file_name_to_string_lossy().to_string(),
            files: vec![],
        };

        let year_lower = args.start_date.year();
        let year_upper = args.end_date.year();

        for dir_year in fs::read_dir(&dir_team)
            .with_context(|| format!("Failed to read dir: {}", &dir_team.display()))?
        {
            // Iterating year folders
            let dir_year = dir_year?.path();
            if dir_year.is_file() {
                // TODO Add test to make sure this works
                eprintln!(
                    "Skipping '{}' because not expecting files at 2nd level only 4 digit year folders",
                    dir_year.display()
                );
                continue;
            }
            let folder_year = dir_year.file_name_to_string_lossy().parse::<i32>();
            match folder_year {
                Ok(year) => {
                    if year_lower <= year && year <= year_upper {
                        result
                            .files
                            .push(InputFile::load_from_disk(dir_year, args)?);
                    }
                }
                Err(e) => {
                    // TODO Add test to make sure this works
                    eprintln!(
                        "Skipping '{}' because unable to parse '{}' into an integer with error: {}",
                        dir_year.display(),
                        &result.name,
                        e
                    );
                }
            }
        }
        Ok(result)
    }
}

pub struct InputFile {
    pub date: NaiveDate,
    pub member_name: String,
    pub is_team_lead: bool,
    pub summary: String,
    pub cancelled: Vec<Task>,
    pub planned: Vec<Task>,
    pub in_progress: Vec<Task>,
    pub complete: Vec<Task>,
}

impl InputFile {
    fn load_from_disk(dir_year: PathBuf, args: &Cli) -> anyhow::Result<Self> {
        todo!()
    }
}

pub struct Task {
    pub name: String,
    pub comment: String, // TODO trim spaces
}
