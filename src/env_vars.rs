//! Environment variable access helpers.

use std::borrow::Cow;
use std::{env, io};

use thiserror::Error;

/// Read the named environment variable.
pub fn var<'a>(name: impl Into<Cow<'a, str>>) -> Result<String, EnvVarError> {
    let name = name.into();
    env::var(name.as_ref()).map_err(|err| EnvVarError::new(name.into_owned(), err))
}

/// Read the named environment variable, or return `default`.
pub fn var_default<'a>(
    name: impl Into<Cow<'a, str>>,
    default: impl Into<String>,
) -> Result<String, EnvVarError> {
    match var(name) {
        Err(EnvVarError {
            name: _,
            error: env::VarError::NotPresent,
        }) => Ok(default.into()),
        Ok(value) if value.is_empty() => Ok(default.into()),
        otherwise => otherwise,
    }
}

/// Wrap [`env::VarError`], including the name of the environment variable.
#[derive(Debug, PartialEq, Eq, Clone)] // same as env::VarError
#[derive(Error)]
#[error("{name}: {error}")]
pub struct EnvVarError {
    name: String,
    error: env::VarError,
}

impl EnvVarError {
    pub fn new(name: String, error: env::VarError) -> Self {
        Self { name, error }
    }
}

/// A default conversion to [`io::Error`], for convenience.
impl From<EnvVarError> for io::Error {
    fn from(err: EnvVarError) -> Self {
        io::Error::new(io::ErrorKind::InvalidData, err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_not_present() {
        let key = "var_not_present";
        env::remove_var(key);
        assert_eq!(
            var(key).unwrap_err().to_string(),
            "var_not_present: environment variable not found"
        );
    }

    #[cfg(unix)]
    #[test]
    fn var_not_unicode() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let key = "var_not_unicode";
        env::set_var(key, OsStr::from_bytes(b"\x80"));
        assert_eq!(
            var(key).unwrap_err().to_string(),
            r#"var_not_unicode: environment variable was not valid unicode: "\x80""#
        );
    }

    #[test]
    fn var_is_empty() {
        let key = "var_is_empty";
        env::set_var(key, "");
        assert_eq!(var(key).unwrap(), "");
    }

    #[test]
    fn var_is_present() {
        let key = "var_is_present";
        env::set_var(key, "present");
        assert_eq!(var(key).unwrap(), "present");
    }

    #[test]
    fn var_default_not_present() {
        let key = "var_default_not_present";
        env::remove_var(key);
        assert_eq!(var_default(key, "default").unwrap(), "default");
    }

    #[test]
    fn var_default_is_empty() {
        let key = "var_default_is_empty";
        env::set_var(key, "");
        assert_eq!(var_default(key, "default").unwrap(), "default");
    }

    #[test]
    fn var_default_is_present() {
        let key = "var_default_is_present";
        env::set_var(key, "present");
        assert_eq!(var_default(key, "default").unwrap(), "present");
    }
}
