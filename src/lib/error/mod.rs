#[derive(Debug)]
pub enum FileHeleperError {
    FileAlreadyExists,
    FileDoesNotExist,
    UnknownError(String),
}

impl std::fmt::Display for FileHeleperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileHeleperError::FileAlreadyExists => write!(f, "file already exists"),
            FileHeleperError::FileDoesNotExist => write!(f, "file does not exist"),
            FileHeleperError::UnknownError(e) => write!(f, "unknown error: {}", e),
        }
    }
}
