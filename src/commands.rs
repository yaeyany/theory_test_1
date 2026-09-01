

pub enum Commands {
    Create,
    Get,
    Update,
    List,
    Delete,
    Shutdown
}

pub enum CommandResponse {
    UnknownCommand,
    CreatedJob,
    InvalidTItle,
    InvalidId,
    InvalidState,
    GetJob,
    UpdatedJob,
    JobList,
    DeletedJob,
    Shutdown
}