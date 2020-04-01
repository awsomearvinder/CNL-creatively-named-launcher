use std::{error, fmt, io};
#[derive(Debug)]
pub enum Errors {
    IoError(io::Error),
    BadName,
    BadExec,
    EmptyFile,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(err) => err.fmt(f),
            Self::BadName => write!(f, "Bad or no Name in .desktop file"),
            Self::BadExec => write!(f, "Bad or no Exec in .desktop file"),
            Self::EmptyFile => write!(f, "Empty file contents."),
        }
    }
}

impl From<io::Error> for Errors {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl std::error::Error for Errors {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::BadName => None,
            Self::BadExec => None,
            Self::EmptyFile => None,
            Self::IoError(err) => Some(err),
        }
    }
}
