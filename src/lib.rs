use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Directories {
    pub home: PathBuf,
    pub data: PathBuf,
    pub config: PathBuf,
    pub cache: PathBuf,
    pub state: Option<PathBuf>,
    pub runtime: Option<PathBuf>,
}

/// A convenience function that wraps the [`home_dir`](https://docs.rs/home/0.5.9/home/fn.home_dir.html) function from the [home](https://docs.rs/home) crate.
pub fn home_dir() -> Result<std::path::PathBuf, HomeDirError> {
    home::home_dir().ok_or(HomeDirError)
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

fn macos(home: &PathBuf) -> Directories {
    let library = home.join("Library");

    Directories {
        home: home.clone(),
        data: library.join("Application Support"),
        config: library.join("Preferences"),
        cache: library.join("Caches"),
        state: None,
        runtime: None,
    }
}

fn windows(home: &PathBuf) -> Directories {
    let appdata =
        env::var("APPDATA").map_or_else(|_| home.join("AppData").join("Roaming"), PathBuf::from);
    let local_appdata =
        env::var("LOCALAPPDATA").map_or_else(|_| home.join("AppData").join("Local"), PathBuf::from);

    Directories {
        home: home.clone(),
        data: appdata.clone(),
        config: appdata,
        cache: local_appdata,
        state: None,
        runtime: None,
    }
}

fn linux(home: &PathBuf) -> Directories {
    Directories {
        home: home.clone(),
        data: home.join(".local").join("share"),
        config: home.join(".config"),
        cache: home.join(".cache"),
        state: Some(home.join(".local").join("state")),
        runtime: None,
    }
}

pub fn dirs() -> Result<Directories, HomeDirError> {
    let home: PathBuf = home_dir()?;

    let mut result = match env::consts::OS {
        "macos" => macos(&home),
        "windows" => windows(&home),
        _ => linux(&home),
    };

    if let Ok(xdg_data) = env::var("XDG_DATA_HOME") {
        result.data = PathBuf::from(xdg_data);
    }
    if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
        result.config = PathBuf::from(xdg_config);
    }
    if let Ok(xdg_cache) = env::var("XDG_CACHE_HOME") {
        result.cache = PathBuf::from(xdg_cache);
    }
    if let Ok(xdg_state) = env::var("XDG_STATE_HOME") {
        result.state = Some(PathBuf::from(xdg_state));
    }
    if let Ok(xdg_runtime) = env::var("XDG_RUNTIME_DIR") {
        result.runtime = Some(PathBuf::from(xdg_runtime));
    }

    Ok(result)
}
