use env_logger::{Builder, Target};
use log::{debug, error, info, trace, warn};

pub fn init() {
    Builder::from_env("RUST_LOG")
        .target(Target::Stdout)
        .init();

    error!("ERROR log enabled");
    warn!("WARN log enabled");
    info!("INFO log enabled");
    debug!("DEBUG log enabled");
    trace!("TRACE log enabled");
}