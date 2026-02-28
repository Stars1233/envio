use std::path::PathBuf;

use envio::profile::{ProfileMetadata, SerializedProfile};

use crate::{
    error::{AppError, AppResult},
    utils::get_cwd,
};

pub fn get_profile_dir() -> AppResult<PathBuf> {
    let envio_dir = get_cwd().join(".envio");
    if !envio_dir.exists() {
        return Err(AppError::Msg(
            "Current directory has no .envio folder, run `envio init` first.".to_string(),
        ));
    }
    Ok(envio_dir.join("profiles"))
}

pub fn contains_path_separator(s: &str) -> bool {
    s.contains('/') || s.contains('\\')
}

/// returns the path for a profile that does **not** exist yet
pub fn build_profile_path(profile_name: &str) -> AppResult<PathBuf> {
    Ok(get_profile_dir()?.join(format!("{profile_name}.envio")))
}

/// returns the path for a profile that **must exist**
pub fn get_profile_path(profile_name: &str) -> AppResult<PathBuf> {
    let path = build_profile_path(profile_name)?;

    if !path.exists() {
        return Err(AppError::ProfileDoesNotExist(profile_name.to_string()));
    }

    Ok(path)
}

pub fn get_profile_metadata(profile_name: &str) -> AppResult<ProfileMetadata> {
    let path = get_profile_path(profile_name)?;
    let serialized_profile: SerializedProfile = envio::utils::get_serialized_profile(path)?;
    Ok(serialized_profile.metadata)
}
