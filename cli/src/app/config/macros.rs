#[macro_export]
macro_rules! generate_key {
    ($key:expr) => {
        &Yaml::String($key.to_string())
    };
}
