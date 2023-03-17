use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;

use thiserror::Error;
use utoipa::OpenApi;

#[derive(Error, Debug)]
pub enum SaveSpecError {
    #[error("VarError: {0}")]
    VarError(String),
    #[error("SpecPrettierError: {0}")]
    SpecPrettierError(String),
    #[error("WriteFileError: {0:?}")]
    WriteFileError(#[from] std::io::Error),
}

type SaveSpecResult = Result<(), SaveSpecError>;
type OsVarResult = Result<OsString, SaveSpecError>;

pub fn save_spec<T: OpenApi>(file_name: &str) -> SaveSpecResult {
    let out_dir = env::var_os("CARGO_MANIFEST_DIR").map_or(
        OsVarResult::Err(SaveSpecError::VarError(
            "Manifest Env Var loading error".to_string(),
        )),
        OsVarResult::Ok,
    )?;
    let dest_path = Path::new(&out_dir).join(file_name);
    let doc = T::openapi();
    let spec = doc.to_pretty_json().map_err(|e| {
        SaveSpecError::SpecPrettierError(format!("Unable to generate spec doc {}", e))
    })?;
    fs::write(dest_path, spec)?;

    Ok(())
}
