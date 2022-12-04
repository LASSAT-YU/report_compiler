use crate::settings::Cli;
use anyhow::Context;
use chrono::NaiveDate;
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

        for file in fs::read_dir(&args.folder)
            .with_context(|| format!("Failed to read dir: {}", &args.folder))?
        {
            // Iterating top level folder, expecting team folders only
            let path = file?.path();
            if path
                .file_name()
                .unwrap_or_else(|| panic!("Failed to get filename for {}", path.display()))
                .to_string_lossy()
                .starts_with('.')
            {
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
            result.teams.push(TeamFiles::load_from_disk(path)?);
        }
        Ok(result)
    }
}

pub struct TeamFiles {
    pub name: String,
    pub files: Vec<InputFile>,
}

impl TeamFiles {
    fn load_from_disk(path: PathBuf) -> anyhow::Result<Self> {
        todo!()
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

pub struct Task {
    pub name: String,
    pub comment: String, // TODO trim spaces
}
