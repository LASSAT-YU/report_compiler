use std::borrow::Cow;
use std::path::PathBuf;

pub trait FileNameExtract {
    /// Gets the filename of the path provided (or dir name if last section is dir).
    ///
    /// # Panics
    ///
    /// Panics if it is not able to get a filename from the path
    fn file_name_to_string_lossy(&self) -> Cow<str>;
}

impl FileNameExtract for PathBuf {
    fn file_name_to_string_lossy(&self) -> Cow<str> {
        self.file_name()
            .unwrap_or_else(|| panic!("Failed to get filename for {}", self.display()))
            .to_string_lossy()
    }
}

pub trait StringUtils {
    /// Appends an end of line to a string (uses '\n')
    fn add_eol(&mut self);

    // Appends an indent (uses 4 spaces, to match PyCharm). `count` indents will be added
    fn add_indent(&mut self, count: usize);
}

impl StringUtils for String {
    fn add_eol(&mut self) {
        self.push('\n');
    }

    fn add_indent(&mut self, count: usize) {
        self.push_str(&"    ".repeat(count));
    }
}
