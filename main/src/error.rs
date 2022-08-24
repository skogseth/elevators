#[derive(Debug)]
pub enum Error {
    StartUp(std::io::Error),
    ChannelShutdown,
    GracefulShutdown,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::StartUp(e) => write!(f, "network error occurred at startup: {e}"),
            Error::ChannelShutdown => write!(f, "channel shutdown"),
            Error::GracefulShutdown => write!(f, "graceful shutdown"),
        }
    }
}

impl std::error::Error for Error {}


pub trait Logger {
    fn log_if_err(self);
}

impl Logger for Result<(), std::io::Error> {
    fn log_if_err(self) {
        if let Err(e) = self {
            eprintln!("{:?}", e);
        }
    }
}
