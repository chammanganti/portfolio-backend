use rocket::fairing::{AdHoc, Fairing};
use std::env;

use crate::jwt::JwtService;

pub struct AppConfig {
    pub jwt_service: JwtService,
}

impl AppConfig {
    pub fn manage() -> impl Fairing {
        AdHoc::on_ignite("App config", |rocket| async move {
            let jwt_secret = Self::get_env_or_default(
                "JWT_SECRET",
                "qgq0s9k/AWXVyfRzgLy6b8my4KWGA4Z29qtFRo09r9Y=",
            );
            let jwt_service = JwtService::new(jwt_secret);

            rocket.manage(AppConfig { jwt_service })
        })
    }

    fn get_env_or_default(key: &str, default_value: &str) -> String {
        env::var(key).unwrap_or_else(|_| {
            if cfg!(debug_assertions) {
                String::from(default_value)
            } else {
                panic!("env variable `{key}` should be added on release mode")
            }
        })
    }
}
