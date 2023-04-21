use crate::error::Serror;
use std::ffi::OsStr;
pub fn env_var<T>(var: T) -> Result<String, Serror>
where
    T: AsRef<OsStr>,
{
    std::env::var(&var).map_err(|_| {
        Serror::Environment({
            if let Some(x) = var.as_ref().to_str() {
                format!("environment variable: {} not found", x)
            } else {
                "environment variable not found".to_string()
            }
        })
    })
}
