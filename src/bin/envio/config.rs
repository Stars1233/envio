use std::path::PathBuf;

use envio::profile::{ProfileMetadata, SerializedProfile};

use crate::{
    error::{AppError, AppResult},
    utils::{get_cwd, get_home_dir},
};

use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, EnumString)]
pub enum ConfigScope {
    #[strum(ascii_case_insensitive, to_string = "local")]
    Local,
    #[strum(ascii_case_insensitive, to_string = "global")]
    Global,
}

pub fn config_dir_for(scope: ConfigScope) -> PathBuf {
    match scope {
        ConfigScope::Local => get_cwd().join(".envio"),
        ConfigScope::Global => get_home_dir().join(".envio"),
    }
}

pub fn get_config_dir() -> PathBuf {
    let local = config_dir_for(ConfigScope::Local);
    if local.exists() {
        local
    } else {
        config_dir_for(ConfigScope::Global)
    }
}

pub fn profile_dir_for(scope: ConfigScope) -> PathBuf {
    config_dir_for(scope).join("profiles")
}

pub fn get_profile_dir() -> PathBuf {
    let local = profile_dir_for(ConfigScope::Local);

    if local.exists() {
        local
    } else {
        profile_dir_for(ConfigScope::Global)
    }
}

#[cfg(target_family = "unix")]
pub fn get_shellscript_path() -> PathBuf {
    get_config_dir().join("setenv.sh")
}

pub fn contains_path_separator(s: &str) -> bool {
    s.contains('/') || s.contains('\\')
}

/// returns the path for a profile that does **not** exist yet
pub fn build_profile_path(profile_name: &str, scope: Option<ConfigScope>) -> PathBuf {
    if let Some(scope) = scope {
        return profile_dir_for(scope).join(format!("{profile_name}.env"));
    }

    get_profile_dir().join(format!("{profile_name}.env"))
}

/// returns the path for a profile that **must exist**
pub fn get_profile_path(profile_name: &str, scope: Option<ConfigScope>) -> AppResult<PathBuf> {
    let path = build_profile_path(profile_name, scope);

    if !path.exists() {
        return Err(AppError::ProfileDoesNotExist(profile_name.to_string()));
    }

    Ok(path)
}

pub fn get_profile_metadata(
    profile_name: &str,
    scope: Option<ConfigScope>,
) -> AppResult<ProfileMetadata> {
    let path = get_profile_path(profile_name, scope)?;
    let serialized_profile: SerializedProfile = envio::utils::get_serialized_profile(path)?;
    Ok(serialized_profile.metadata)
}
