use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Default, Debug, Clone)]
pub struct FileInfo {
    pub path: Option<PathBuf>,
}

impl FileInfo {
    pub fn from(file_name: &str) -> Self {
        Self {
            path: Some(PathBuf::from(file_name)),
        }
    }
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self
            .path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "[No Name]".into());
        write!(f, "{}", name)
    }
}
