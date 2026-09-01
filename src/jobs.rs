
pub struct Job {
    id: JobId,
    title: JobTitle,
    state: JobState
}

pub struct JobId(u32);

pub enum JobState{
    New,
    InProgress,
    Done
}

pub struct JobTitle(String);

