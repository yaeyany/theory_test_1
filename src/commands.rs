use std::sync::mpsc::Sender;

use crate::jobs::{JobId, JobState, JobTitle};



pub enum Commands {
    Create {
        title: JobTitle,
        response_sender: Sender<CommandResponse>
    },
    Get {
        id: JobId,
        response_sender: Sender<CommandResponse>
    },
    Update {
        id: JobId,
        title: Option<JobTitle>,
        state: Option<JobState>,
        response_sender: Sender<CommandResponse>
    },
    List {
        response_sender: Sender<CommandResponse>
    },
    Delete {
        id: JobId,
        response_sender: Sender<CommandResponse>
    },
    Shutdown {
        response_sender: Sender<CommandResponse>
    }
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