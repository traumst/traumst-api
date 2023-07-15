use std::env;

pub fn listen_on_port() -> u16 {
    match env::var("LISTEN_PORT") {
        Ok(val) => val.parse().expect("Invalid LISTEN_PORT in env"),
        Err(_e) => panic!("LISTEN_PORT was not specified in env")
    }
}

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