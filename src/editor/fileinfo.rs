use std::{
    fmt::{self, Display},
    path::PathBuf,
};

const NO_NAME: &str = "[No Name]";

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
        #[allow(clippy::map_unwrap_or)]
        let name = self
            .path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| NO_NAME.into());
        write!(f, "{name}")
    }
}
