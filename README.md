# user_dirs

A respectful implementation of user directories for Rust.

The library provides the location of these directories by leveraging the mechanisms defined by

- the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) on all platforms primarily,
- the [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/dd378457.aspx) API on Windows,
- and the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6) guidelines on macOS.

The key differentiator of the `user_dirs` crate is that it always prefers user-defined XDG variables, regardless of platform. Then and only then are system-defined directories used.

## Examples

All directories are based off of the user's home directory, which is provided by the [`home`](https://docs.rs/home) crate. The home directory can be accessed through the `home` field of `user_dirs::Directories`, or directly from the exported `user_dirs::home_dir` wrapper function used internally.

For a user named Alice:

```rs
let dirs = user_dirs::dirs().unwrap();
// => user_dirs::Directories

dirs.home;
// (See above.)

dirs.cache;
// XDG? => $XDG_CACHE_HOME
// macOS => /Users/Alice/Library/Caches
// Windows => C:\Users\Alice\AppData\Local
// Linux => /home/alice/.cache

dirs.config;
// XDG? => $XDG_CONFIG_HOME
// macOS => /Users/Alice/Library/Preferences
// Windows => C:\Users\Alice\AppData\Roaming
// Linux => /home/alice/.config

dirs.data;
// XDG? => $XDG_DATA_HOME
// macOS => /Users/Alice/Library/Application Support
// Windows => C:\Users\Alice\AppData\Roaming
// Linux => /home/alice/.local/share

dirs.runtime;
// XDG? => Some($XDG_RUNTIME_DIR)
// macOS => None
// Windows => None
// Linux => None

dirs.state;
// XDG? => Some($XDG_STATE_HOME)
// macOS => None
// Windows => None
// Linux => Some(/home/alice/.local/state)
```

## License

[MIT](LICENSE)
