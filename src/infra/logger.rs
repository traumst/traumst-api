use env_logger::{Builder, Target};

pub fn init() {
    Builder::from_env("RUST_LOG")
        .target(Target::Stdout)
        .init();
}