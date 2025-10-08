use log::{
    info,
};
use std::env;
use std::fs::read_to_string;


pub struct Config {
    pub listen_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub owner_email: String,
}

pub fn load() -> Config {
    let mut config = Config { listen_port: 7654, smtp_pass: "".to_string(), smtp_user: "".to_string(), owner_email: "".to_string(), };
    match read_to_string(".env") {
        Ok(raw_val) => {
            for line in raw_val.lines() {
                info!("{}", line);
            }
        },
        Err(e) => panic!("Could not find .env file: {}", e),
    }
    return config;
}

pub fn listen_on_port() -> u16 {
    match env::var("LISTEN_PORT") {
        Ok(val) => val.parse().expect("Invalid LISTEN_PORT in env"),
        Err(e) => panic!("LISTEN_PORT was not specified in env: {}", e)
    }
}

pub fn smtp_user() -> String { env::var("SMTP_USER").expect("SMTP_USER is not set") }
pub fn smtp_pass() -> String { env::var("SMTP_PASS").expect("SMTP_PASS is not set") }
pub fn email_receiver() -> String { env::var("EMAIL_RECEIVER").expect("EMAIL_RECEIVER is not set") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listen_on_port() {
        env::set_var("LISTEN_PORT", "65535");
        assert_eq!(listen_on_port(), 65535);
    }

    #[test]
    #[should_panic]
    fn test_bad_listen_on_port() {
        env::remove_var("LISTEN_PORT");
        listen_on_port();
    }
}