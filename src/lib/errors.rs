use std::{error, ffi, fmt, io};
#[derive(Debug)]
pub enum Errors {
    IoError(io::Error),
    BadName,
    BadExec,
    NotValidUtf8(ffi::OsString),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(err) => err.fmt(f),
            Self::BadName => write!(f, "Bad or no Name in .desktop file"),
            Self::BadExec => write!(f, "Bad or no Exec in .desktop file"),
            Self::NotValidUtf8(err) => write!(f, "OS string is not valid utf8, string: {:#?}", err),
        }
    }
}

impl From<io::Error> for Errors {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}
impl From<ffi::OsString> for Errors {
    fn from(err: ffi::OsString) -> Self {
        Self::NotValidUtf8(err)
    }
}

impl std::error::Error for Errors {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::BadName => None,
            Self::BadExec => None,
            Self::IoError(err) => Some(err),
            Self::NotValidUtf8(_) => None,
        }
    }
}
