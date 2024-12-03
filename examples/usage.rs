fn main() {
    println!("{:?}", user_dirs::config_dir());

    std::env::remove_var("XDG_CONFIG_HOME");

    assert_eq!(
        user_dirs::config_dir().unwrap(),
        user_dirs::os::config_dir().unwrap()
    );

    println!("{:?}", user_dirs::config_dir());
}
