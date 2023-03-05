use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Indices(err: std::string::String) {
            display("{}", err)
        }
    }
}
