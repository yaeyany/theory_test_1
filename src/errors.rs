use core::error;



#[derive(thiserror::Error, Debug)]
pub enum CustomErrors {
    #[error("Unknown command")]
    CommandError,
    
}