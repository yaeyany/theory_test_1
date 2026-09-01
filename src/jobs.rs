use std::{collections::BTreeMap, ops::AddAssign};


pub struct Job {
    title: JobTitle,
    state: JobState
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct JobId(u32);

pub enum JobState{
    New,
    InProgress,
    Done
}

pub struct JobTitle(String);

pub struct JobStore {
    id: JobId,
    jobs: BTreeMap<JobId, Job>
}

impl AddAssign<u32> for JobId {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs
    }
}

impl JobStore {
    pub fn new() -> Self {
        Self { 
            id: JobId(0), 
            jobs: BTreeMap::new() 
        }
    }

    pub fn add(&mut self, title: JobTitle) -> JobId {
        let id = self.id;
        let job = Job { 
            title: title, 
            state: JobState::New 
        };
        
        self.jobs
        .insert(id, job);
        
        self.id += 1;
        id
    }
}

