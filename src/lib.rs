//! A respectful implementation of user directories for Rust.
//!
//! The library provides the location of these directories by leveraging the mechanisms defined by
//!
//! - the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) on all platforms primarily,
//! - the [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/dd378457.aspx) API on Windows,
//! - and the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6) guidelines on macOS.
//!
//! ## Why?
//!
//! The key differentiator of the `user_dirs` crate is that it always prefers user-defined XDG variables, regardless of platform. Then and only then are system-defined directories used. The developer of the popular `dirs` and `directories` crates [has refused time and time again to respect explicitly-defined XDG variables](https://github.com/dirs-dev/directories-rs/issues/47#issuecomment-478337412).
//!
//! I'm a big fan of the [`etcetera`](https://docs.rs/etcetera/latest/etcetera/) library, however it overcomplicates the process by providing multiple strategies; `user_dirs` picks one.
//!
//! ## Examples
//!
//! For a user named Leah:
//!
//! ```rust
//! user_dirs::home_dir();
//! // See the [`home`](https://docs.rs/home) crate.
//!
//! user_dirs::cache_dir();
//! // XDG? => $XDG_CACHE_HOME
//! // macOS => /Users/Leah/Library/Caches
//! // Windows => C:\Users\Leah\AppData\Local
//! // Linux => /home/leah/.cache
//!
//! user_dirs::config_dir();
//! // XDG? => $XDG_CONFIG_HOME
//! // macOS => /Users/Leah/Library/Preferences
//! // Windows => C:\Users\Leah\AppData\Roaming
//! // Linux => /home/leah/.config
//!
//! user_dirs::data_dir();
//! // XDG? => $XDG_DATA_HOME
//! // macOS => /Users/Leah/Library/Application Support
//! // Windows => C:\Users\Leah\AppData\Roaming
//! // Linux => /home/leah/.local/share
//!
//! user_dirs::runtime_dir();
//! // XDG? => Some($XDG_RUNTIME_DIR)
//! // macOS => None
//! // Windows => None
//! // Linux => None
//!
//! user_dirs::state_dir();
//! // XDG? => Some($XDG_STATE_HOME)
//! // macOS => None
//! // Windows => None
//! // Linux => Some(/home/leah/.local/state)
//! ```

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
