# user_dirs

A respectful, XDG-first user directories implementation for obtaining the home, cache, config, data, runtime, and state directories.

The library provides the location of these directories by leveraging the mechanisms defined by

- the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) on all platforms primarily,
- the [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/dd378457.aspx) API on Windows,
- and the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6) guidelines on macOS.

## Why?

The key differentiator of the `user_dirs` crate is that it always prefers user-defined XDG variables, regardless of platform. Then and only then are system-defined directories used. The developer of the popular `dirs` and `directories` crates [has refused time and time again to respect explicitly-defined XDG variables](https://github.com/dirs-dev/directories-rs/issues/47#issuecomment-478337412).

I'm a big fan of the [`etcetera`](https://docs.rs/etcetera/latest/etcetera/) library, however it overcomplicates the process by providing multiple strategies; `user_dirs` picks one.

## Examples

For a user named Leah:

```rust
user_dirs::home_dir();
// See the [`home`](https://docs.rs/home) crate.

user_dirs::cache_dir();
// XDG? => $XDG_CACHE_HOME
// macOS => /Users/Leah/Library/Caches
// Windows => C:\Users\Leah\AppData\Local
// Linux => /home/leah/.cache

user_dirs::config_dir();
// XDG? => $XDG_CONFIG_HOME
// macOS => /Users/Leah/Library/Preferences
// Windows => C:\Users\Leah\AppData\Roaming
// Linux => /home/leah/.config

user_dirs::data_dir();
// XDG? => $XDG_DATA_HOME
// macOS => /Users/Leah/Library/Application Support
// Windows => C:\Users\Leah\AppData\Roaming
// Linux => /home/leah/.local/share

user_dirs::runtime_dir();
// XDG? => Some($XDG_RUNTIME_DIR)
// macOS => None
// Windows => None
// Linux => None

user_dirs::state_dir();
// XDG? => Some($XDG_STATE_HOME)
// macOS => None
// Windows => None
// Linux => Some(/home/leah/.local/state)
```

## License

[MIT](LICENSE)
