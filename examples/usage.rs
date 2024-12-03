fn main() {
    println!("{:?}", user_dirs::config_dir());

    std::env::remove_var("XDG_CONFIG_HOME");

    println!("{:?}", user_dirs::config_dir());
}
