use std::env;

pub enum Environment {
    Production,
    Stagging,
    Testing,
    Development,
}

pub fn get_environment() -> Environment {
    match env::var("APP_ENV").ok().as_deref() {
        Some("production") => Environment::Production,
        Some("stagging") => Environment::Stagging,
        Some("testing") => Environment::Testing,
        _ => Environment::Development,
    }
}
