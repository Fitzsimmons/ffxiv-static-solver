use std::collections::HashMap;
use std::fmt;
use std::error::Error;
use std::hash::{Hash, Hasher};

use crate::Job;

#[derive(Clone, PartialOrd, Ord, Eq)]
pub struct Slot<'a> {
    role: String,
    role_jobs: &'a Vec<Job>,
    player_name: Option<String>,
    rank: Option<usize>,
    job: Option<Job>,
}

impl Slot<'_> {
    pub fn new(role: String, definitions: &HashMap<String, Vec<Job>>) -> Result<Slot, Box<dyn Error>> {
        let role_jobs = definitions.get(&role).ok_or(format!("Role {} not found", role))?;

        return Ok(Slot {
            role_jobs,
            role,
            player_name: None,
            rank: None,
            job: None,
        })
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

impl fmt::Display for Slot<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t: (String, String) = self.into();
        write!(f, "{}: {}", t.0, t.1)
    }
}

impl PartialEq for Slot<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.role == other.role &&
        self.player_name == other.player_name &&
        self.rank == other.rank &&
        self.job == other.job
    }
}

impl Hash for Slot<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.role.hash(state);
        self.player_name.hash(state);
        self.rank.hash(state);
        self.job.hash(state);
    }
}

impl From<&Slot<'_>> for (String, String) {
    fn from(item: &Slot<'_>) -> (String, String) {
        if item.player_name.is_some() {
            (item.player_name.clone().unwrap(), item.job.unwrap().to_string())
        } else {
            (String::from("Empty Slot"), item.role.clone())
        }
    }
}
