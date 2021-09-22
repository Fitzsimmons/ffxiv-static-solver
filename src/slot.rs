use std::fmt;
use std::error::Error;

use crate::Job;

#[derive(Clone, PartialEq, Eq)]
pub struct Slot {
    role: String,
    role_jobs: Vec<Job>, // TODO: can I make this a reference with a lifetime?
    player_name: Option<String>,
    rank: Option<usize>,
    job: Option<Job>,
}

impl Slot {
    pub fn new(role: String) -> Slot {
        return Slot{
            role_jobs: crate::DEFINITIONS.get().unwrap().get(&role).unwrap().clone(),
            role,
            player_name: None, 
            rank: None, 
            job: None,
        }
    }

    pub fn satisfied_by(&self, job: Job) -> bool {
        self.player_name.is_none() && self.role_jobs.contains(&job)
    }

    pub fn assign(&mut self, job: Job, player_name: String, rank: usize) -> Result<(), Box<dyn Error>> {
        if !self.satisfied_by(job) {
            Err("Assignment failed, wrong job or slot already taken")?;
        }

        self.job = Some(job);
        self.player_name = Some(player_name);
        self.rank = Some(rank);
        Ok(())
    }

    pub fn unassign(&mut self) {
        self.player_name = None;
        self.rank = None;
        self.job = None;
    }

    pub fn rank(&self) -> Option<usize> {
        self.rank
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.player_name.is_some() {
            write!(f, "{}: {}", self.player_name.as_ref().unwrap(), self.job.unwrap())
        } else {
            write!(f, "Empty slot: {}", self.role)
        }
    }
}
