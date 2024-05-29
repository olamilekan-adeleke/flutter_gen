#[derive(Debug)]
#[allow(dead_code)]
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

#[derive(Debug)]
pub enum AppError {
    FileCreationError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::FileCreationError(e) => write!(f, "file creation error: {}", e),
        }
    }
}
