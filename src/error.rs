use quick_error::quick_error;

quick_error! {
    #[derive(Debug, Clone)]
    pub enum Error {
        Indices(err: std::string::String) {
            display("{}", err)
        }
        IO(err: std::string::String) {
            display("{}", err)
        }
        InvalidArgument(err: std::string::String) {
            display("{}", err)
        }
        InvalidFormat(err: std::string::String) {
            display("{}", err)
        }
        Internal(err: std::string::String) {
            display("{}", err)
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(format!("IO Error: {}", err))
    }
}