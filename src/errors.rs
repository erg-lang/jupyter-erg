use std::fmt;
use std::io;

use erg_compiler::error::CompileError;

#[derive(Debug)]
pub enum Error {
    CompilationErrors(Vec<CompileError>),
    Message(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CompilationErrors(errors) => {
                for error in errors {
                    write!(f, "{}", error.message())?;
                }
            }
            /*Error::TypeRedefinedVariablesLost(variables) => {
                write!(
                    f,
                    "A type redefinition resulted in the following variables being lost: {}",
                    variables.join(", ")
                )?;
            }*/
            Error::Message(message) => { // | Error::SubprocessTerminated(message) => {
                write!(f, "{}", message)?
            }
        }
        Ok(())
    }
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Message(error.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Message(error.to_string())
    }
}

impl From<json::Error> for Error {
    fn from(error: json::Error) -> Self {
        Error::Message(error.to_string())
    }
}

impl<'a> From<&'a io::Error> for Error {
    fn from(error: &'a io::Error) -> Self {
        Error::Message(error.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::Message(error.to_string())
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::Message(message)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(message: &str) -> Self {
        Error::Message(message.to_owned())
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Error::Message(error.to_string())
    }
}
