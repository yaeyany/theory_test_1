

#[derive(thiserror::Error, Debug)]
pub enum CustomErrors {
    #[error("Unknown command")]
    CommandError,
    #[error("Server unavailable")]
    ServerUnavailable,
    #[error("Workers unavailable")]
    WorkersUnavailable,
}

pub fn handle_error(error: CustomErrors) {
    println!("{}", error)
}

