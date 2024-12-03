fn main() {
    println!("{:?}", user_dirs::dirs().unwrap());

    std::env::remove_var("XDG_CONFIG_HOME");
    std::env::remove_var("XDG_DATA_HOME");
    std::env::remove_var("XDG_CACHE_HOME");
    std::env::remove_var("XDG_STATE_HOME");
    std::env::remove_var("XDG_RUNTIME_DIR");

    println!("{:?}", user_dirs::dirs().unwrap());
}
