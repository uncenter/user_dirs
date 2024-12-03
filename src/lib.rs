use std::env;
use std::path::PathBuf;

/// Returns the path to the home directory.
pub fn home_dir() -> Result<PathBuf, HomeDirError> {
    home::home_dir().ok_or(HomeDirError)
}

/// Returns the path to the data directory.
pub fn data_dir() -> Result<PathBuf, HomeDirError> {
    let dir = if let Ok(xdg_data) = env::var("XDG_DATA_HOME") {
        PathBuf::from(xdg_data)
    } else {
        let home = home_dir()?;

        match env::consts::OS {
            "macos" => home.join("Library").join("Application Support"),
            "windows" => env::var("APPDATA")
                .map_or_else(|_| home.join("AppData").join("Roaming"), PathBuf::from),
            _ => home.join(".local").join("share"),
        }
    };

    Ok(dir)
}

/// Returns the path to the config directory.
pub fn config_dir() -> Result<PathBuf, HomeDirError> {
    let dir = if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg_config)
    } else {
        let home = home_dir()?;

        match env::consts::OS {
            "macos" => home.join("Library").join("Preferences"),
            "windows" => env::var("APPDATA")
                .map_or_else(|_| home.join("AppData").join("Roaming"), PathBuf::from),
            _ => home.join(".config"),
        }
    };
    Ok(dir)
}

/// Returns the path to the cache directory.
pub fn cache_dir() -> Result<PathBuf, HomeDirError> {
    let dir = if let Ok(xdg_cache) = env::var("XDG_CACHE_HOME") {
        PathBuf::from(xdg_cache)
    } else {
        let home = home_dir()?;

        match env::consts::OS {
            "macos" => home.join("Library").join("Caches"),
            "windows" => env::var("LOCALAPPDATA")
                .map_or_else(|_| home.join("AppData").join("Local"), PathBuf::from),
            _ => home.join(".cache"),
        }
    };
    Ok(dir)
}

/// Returns the path to the state directory, if available.
pub fn state_dir() -> Result<Option<PathBuf>, HomeDirError> {
    let dir = if let Ok(xdg_state) = env::var("XDG_STATE_HOME") {
        Some(PathBuf::from(xdg_state))
    } else {
        let home = home_dir()?;

        Some(match env::consts::OS {
            "macos" | "windows" => return Ok(None), // No state directory on macOS or Windows by default.
            _ => home.join(".local").join("state"),
        })
    };

    Ok(dir)
}

/// Returns the path to the runtime directory, if available.
pub fn runtime_dir() -> Option<PathBuf> {
    if let Ok(xdg_runtime) = env::var("XDG_RUNTIME_DIR") {
        return Some(PathBuf::from(xdg_runtime));
    }
    None
}

/// This error occurs when the home directory cannot be located.
#[derive(Debug)]
pub struct HomeDirError;
impl std::fmt::Display for HomeDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not locate home directory")
    }
}
impl std::error::Error for HomeDirError {}
