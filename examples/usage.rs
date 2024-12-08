fn main() {
    std::env::set_var("XDG_CONFIG_HOME", "foo");
    println!("With XDG_CONFIG_HOME set: {:?}", user_dirs::config_dir());

    std::env::remove_var("XDG_CONFIG_HOME");
    println!("With XDG_CONFIG_HOME unset: {:?}", user_dirs::config_dir());
    assert_eq!(
        user_dirs::config_dir().unwrap(),
        user_dirs::os::config_dir().unwrap()
    );
}
