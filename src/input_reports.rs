use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::slice::Iter;

use anyhow::{anyhow, Context};
use chrono::{Datelike, NaiveDate};
use regex::Regex;

use crate::settings::Cli;
use crate::utils::{FileNameExtract, StringUtils};

pub fn load_input_files(args: &Cli) -> anyhow::Result<AllInputFiles> {
    AllInputFiles::load_from_disk(args)
}

pub struct AllInputFiles {
    pub teams: Vec<TeamFiles>,
}

impl AllInputFiles {
    pub fn iter(&self) -> Iter<TeamFiles> {
        self.teams.iter()
    }

    fn load_from_disk(args: &Cli) -> anyhow::Result<Self> {
        let mut result = Self { teams: vec![] };

        for entry in fs::read_dir(&args.folder)
            .with_context(|| format!("Failed to read dir: '{}'", &args.folder))?
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
        result.sort();
        Ok(result)
    }
    fn sort(&mut self) {
        self.teams.sort();
    }
}

impl<'a> IntoIterator for &'a AllInputFiles {
    type Item = &'a TeamFiles;
    type IntoIter = Iter<'a, TeamFiles>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Groups all files for a given team.
///
/// # Invariant
///
/// 1. [`TeamFiles::files`] is sorted
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TeamFiles {
    pub name: String,
    pub files: Vec<InputFile>,
}

impl TeamFiles {
    pub fn tasks(&self) -> Vec<Task> {
        let mut result = vec![];
        let member_list = self.files_by_member();
        let files = member_list.iter().flat_map(|member| member.iter());
        for file in files {
            let sections = vec![
                &file.cancelled,
                &file.planned,
                &file.in_progress,
                &file.complete,
            ];
            for t in sections.iter().flat_map(|&x| x.iter()) {
                // TODO Find a better way instead of copying tasks
                result.push(t.clone());
            }
        }
        result
    }

    /// Returns one struct per member with all the files for that member. Makes use of invariant
    /// on TeamFiles that [`TeamFiles::files`] is sorted (This would mean that all files for the
    /// same member are sequential)
    pub fn files_by_member(&self) -> Vec<TeamMember> {
        let mut result = vec![];
        let mut iter = self.iter().enumerate();
        let (mut start_index, mut curr_member_display_name) = match iter.next() {
            None => return vec![],
            Some((i, file)) => (i, file.member_display_name()),
        };

        // Main loop
        for (i, file) in iter {
            if file.member_display_name() != curr_member_display_name {
                result.push(TeamMember {
                    files: &self.files[start_index..i],
                });
                start_index = i;
                curr_member_display_name = file.member_display_name();
            }
        }

        // Add last set of files
        // TODO Add test for team with only 1 member and no members
        result.push(TeamMember {
            files: &self.files[start_index..],
        });
        result
    }

    pub fn iter(&self) -> Iter<InputFile> {
        self.files.iter()
    }

    pub fn sort(&mut self) {
        self.files.sort();
    }

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
                        result.add_files_from_year_dir(dir_year, args)?;
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
        result.sort();
        Ok(result)
    }
    fn add_files_from_year_dir(&mut self, dir_year: PathBuf, args: &Cli) -> anyhow::Result<()> {
        let start_date = &args.start_date;
        let end_date = &args.end_date;

        for file_entry in fs::read_dir(&dir_year)
            .with_context(|| format!("Failed to read dir: {}", &dir_year.display()))?
        {
            // Iterating Individual files and determining if they are included or not
            let file = file_entry?.path();
            if file.is_dir() {
                // TODO Add test to make sure this works
                eprintln!(
                    "Skipping '{}' because not expecting directories at 3rd level only files",
                    dir_year.display()
                );
                continue;
            }
            let file_name = file.file_name_to_string_lossy();
            // ASSUMPTION: Filename is prefixed with date in format YYYY-MM-DD
            if file_name.len() < 10 {
                eprintln!("Skipping '{file_name}' because filename is too short to have a valid date at the start");
                continue;
            }
            let date_part_of_filename = &file_name[0..10];
            match NaiveDate::parse_from_str(date_part_of_filename, "%F") {
                Ok(date) => {
                    if *start_date <= date && date <= *end_date {
                        let input_file = InputFile::load_from_disk(&file, args);
                        match input_file {
                            Ok(value) => {
                                self.files.push(value);
                            }
                            Err(e) => {
                                eprintln!(
                                    "Failed to process {}, due to the following error {:?}",
                                    &file.display(),
                                    e
                                )
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Skipping '{}' because unable to parse '{}' into an date with error: {}",
                        file.display(),
                        &date_part_of_filename,
                        e
                    )
                }
            }
        }
        Ok(())
    }
}

impl<'a> IntoIterator for &'a TeamFiles {
    type Item = &'a InputFile;
    type IntoIter = Iter<'a, InputFile>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
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
    pub fn member_display_name(&self) -> Cow<str> {
        if self.is_team_lead {
            Cow::from(format!("{} `Team Lead`", &self.member_name))
        } else {
            Cow::from(&self.member_name)
        }
    }

    pub fn load_from_disk(file: &PathBuf, _args: &Cli) -> anyhow::Result<Self> {
        let file_name = file.file_name_to_string_lossy();

        // TODO Regex compile to change to lazy static
        let re_file_name = Regex::new(r"^(\d{4}-\d\d-\d\d)_([A-z][a-z]+[A-Z])_?(TL)?.md$")
            .expect("Failed to compile regex");
        let re_new_task =
            Regex::new(r"^- +(?:([\w '()\[\],]+?):)?\s*(.*)$").expect("Failed to compile regex");

        let caps = re_file_name.captures(&file_name);
        if caps.is_none() {
            return Err(anyhow!(
                "Failed to parse '{file_name}'. Doesn't match expected format"
            ));
        }
        let caps = caps.unwrap();
        // ASSUMPTION: Using unwrap on the captures because they are required parts of the regex
        let mut result = Self {
            date: NaiveDate::parse_from_str(caps.get(1).unwrap().as_str(), "%F")?,
            member_name: caps.get(2).unwrap().as_str().to_owned(),
            is_team_lead: caps.get(3).is_some(),
            summary: "".to_string(),
            cancelled: vec![],
            planned: vec![],
            in_progress: vec![],
            complete: vec![],
        };

        // Setup to read file contents
        use InputFileSections as ifs;
        let contents = fs::read_to_string(file)?;
        let mut state = ifs::Summary;
        let mut current_task: Option<Task> = None;
        let mut lines = contents.lines();

        // Check that first line in the heading for the first section
        {
            let first_line = lines.next();
            if first_line.is_none() {
                return Err(anyhow!("File line is empty"));
            }
            let first_line = first_line.unwrap();
            let first_section_heading = Self::get_section_heading(first_line);
            match first_section_heading {
                None => {
                    return Err(anyhow!("Expected first line to be section heading for '{}' but no section heading found",state.section_name()));
                }
                Some(value) => {
                    if value != state.section_name() {
                        return Err(anyhow!(
                            "Expected first section to be '{}' but found {}",
                            state.section_name(),
                            value
                        ));
                    }
                }
            }
        }

        // Parse rest of file starting with current section
        for (i, line) in lines.enumerate() {
            let new_section_heading = Self::get_section_heading(line);
            let curr_vec = match state {
                InputFileSections::Summary => {
                    match new_section_heading {
                        None => {
                            if line.is_empty() {
                                result.summary.add_eol_max_2()
                            } else {
                                writeln!(result.summary, "{}", line.trim()).unwrap();
                            }
                        }
                        Some(value) => {
                            result.summary = result.summary.trim().to_owned();
                            state = state
                                .next()
                                .expect("Internal Error: Summary not expected to be last state");
                            if value != state.section_name() {
                                return Err(anyhow!("Next section heading does not match the expected value. Expected {} but got {value}",state.section_name()));
                            }
                        }
                    }
                    continue;
                }
                InputFileSections::Cancelled => &mut result.cancelled,
                InputFileSections::Planned => &mut result.planned,
                InputFileSections::InProgress => &mut result.in_progress,
                InputFileSections::Complete => {
                    if let Some(value) = new_section_heading {
                        return Err(anyhow!(
                            "Expected {} to be the last section but got a new section {value}",
                            state.section_name()
                        ));
                    }
                    &mut result.complete
                }
            };
            if let Some(value) = new_section_heading {
                state = state.next().unwrap_or_else(|| {
                    panic!(
                        "Internal Error: {} not expected to be last state",
                        state.section_name()
                    )
                });
                if value != state.section_name() {
                    return Err(anyhow!("Next section heading does not match the expected value. Expected {} but got {value}",state.section_name()));
                }
                if let Some(value) = current_task {
                    curr_vec.push(value);
                }
                current_task = None;
                continue;
            }

            let new_task = re_new_task.captures(line);

            if let Some(new_task_value) = new_task {
                if let Some(curr_task_value) = current_task {
                    curr_vec.push(curr_task_value)
                }

                let name = match new_task_value.get(1) {
                    // TODO Add test for name not found
                    None => return Err(anyhow!("Name not fond for task in '{line}', on line {i}")),
                    Some(value) => value.as_str().trim_end().to_string(),
                };
                let comment = match new_task_value.get(2) {
                    None => "".to_string(),
                    Some(value) => value.as_str().trim_end().to_string(),
                };

                current_task = Some(Task {
                    name,
                    comment,
                    section: state,
                });
            } else {
                // No new task adding to existing task
                if !line.is_empty() {
                    if let Some(mut value) = current_task {
                        write!(value.comment, " {}", line.trim()).unwrap();
                        current_task = Some(value);
                    } else {
                        return Err(anyhow!(
                            "Found data on line {i} that doesn't seem to belong to any task: '{}'",
                            line
                        ));
                    }
                }
            }
        }

        // TODO Add tests for ALL assumptions (each with a name to match the type of assumption)
        // ASSUMPTION: Last section is 'completed'
        if let Some(value) = current_task {
            result.complete.push(value);
        }
        Ok(result)
    }
    fn get_section_heading(line: &str) -> Option<&str> {
        // TODO Regex compile to change to lazy static
        let re = Regex::new(r"^# +([\w ]+)$").expect("Failed to compile regex");
        let caps = re.captures(line);

        match caps {
            None => None,
            Some(value) => Some(value.get(1)
                .expect(
                    "Expected capture group 1 to always be available as it is required to match",
                )
                .as_str().trim())
        }
    }
}

impl PartialOrd<Self> for InputFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InputFile {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_team_lead != other.is_team_lead {
            // other and self swapped places from normal comparison intentionally
            return other.is_team_lead.cmp(&self.is_team_lead);
        }
        if self.date != other.date {
            return self.date.cmp(&other.date);
        }
        if self.member_name != other.member_name {
            return self.member_name.cmp(&other.member_name);
        }
        Ordering::Equal
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum InputFileSections {
    #[default]
    Summary,
    Cancelled,
    Planned,
    InProgress,
    Complete,
}

impl InputFileSections {
    pub fn display_name(&self) -> &str {
        match self {
            InputFileSections::Cancelled => "Cancelled",
            InputFileSections::Planned => "Planned",
            InputFileSections::InProgress => "InProgress",
            InputFileSections::Complete => "Completed",
            _ => {
                eprintln!(
                    "Got an attempt to get a display name a unexpected section: {}",
                    self.section_name()
                );
                "NOT APPLICABLE"
            }
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            Self::Summary => Some(Self::Cancelled),
            Self::Cancelled => Some(Self::Planned),
            Self::Planned => Some(Self::InProgress),
            Self::InProgress => Some(Self::Complete),
            Self::Complete => None,
        }
    }
    pub fn section_name(&self) -> &'static str {
        match self {
            InputFileSections::Summary => "Summary",
            InputFileSections::Cancelled => "Cancelled Tasks",
            InputFileSections::Planned => "Planned Tasks",
            InputFileSections::InProgress => "In Progress Tasks",
            InputFileSections::Complete => "Completed Tasks",
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Task {
    pub name: String,
    pub comment: String,
    pub section: InputFileSections,
}

impl Task {
    pub fn display_name(&self) -> String {
        format!("{} `{}`", self.name, self.section.display_name())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
/// Stores files for a member. All files must be for the same member and there must be
/// at least 1 file
pub struct TeamMember<'a> {
    files: &'a [InputFile],
}

impl<'a> TeamMember<'a> {
    pub fn iter(&self) -> Iter<InputFile> {
        self.files.iter()
    }

    /// Makes use of the invariants on [`TeamMember`] that the slice is not empty and all reports are for the same member
    pub fn display_name(&self) -> Cow<str> {
        self.files[0].member_display_name()
    }

    pub fn get_files(&self) -> &[InputFile] {
        self.files
    }

    /// Create a new instance of [`Self`] and requires that `files` is not empty
    pub fn new(files: &'a [InputFile]) -> Self {
        // TODO Assert all equal in debug builds
        assert!(!files.is_empty());
        Self { files }
    }
}
