use std::error::Error;
use std::fmt;

use serde_json::Value;

use crate::Job;
use crate::Player;
use crate::slot::Slot;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq)]
pub struct Slots<'a> {
    pub slots: Vec<Slot<'a>>,
    claimed_jobs: Vec<Job>,
}

impl Slots<'_> {
    pub fn new<'a>(composition: &serde_json::Map<String, Value>, definitions: &'a HashMap<String, Vec<Job>>) -> Result<Slots<'a>, Box<dyn Error>> {
        let mut slots = Vec::new();

        for (role, raw_count) in composition.iter() {
            let count = raw_count.as_u64().ok_or(format!("Expected value for {} to be a number", role))?;
            for _ in 0..count {
                slots.push(Slot::new(role.clone(), definitions)?)
            }
        }

        Ok(Slots{claimed_jobs: Vec::new(), slots})
    }

    pub fn assign(&mut self, player: &Player) -> Result<(), Box<dyn Error>> {
        for (rank, &job) in player.jobs.iter().enumerate() {
            if self.claimed_jobs.contains(&job) { continue; }

            let placement_slot = self.slots.iter_mut().find(|slot| slot.satisfied_by(job));

            if placement_slot.is_some() {
                placement_slot.unwrap().assign(job, player.name.clone(), rank)?;
                self.claimed_jobs.push(job);
                return Ok(());
            }
        }

        Err(format!("Could not find slot for {}", player.name))?
    }

    pub fn unassign(&mut self) {
        self.slots.iter_mut().for_each(|s| s.unassign());
        self.claimed_jobs.clear();
    }

    pub fn mean_squared_error(&self) -> Option<f64> {
        self.slots.iter()
            .map(|s| s.rank())
            .into_iter()
            .sum::<Option<usize>>()
            .map(|sum| (sum * sum) as f64 / self.slots.len() as f64)
    }

    pub fn sort(&mut self) {
        self.slots.sort();
    }
}

impl PartialEq for Slots<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.slots == other.slots
    }
}

impl fmt::Display for Slots<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for slot in &self.slots {
            write!(f, "{}\n", slot)?;
        }
        Ok(())
    }
}

impl Hash for Slots<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.slots.hash(state);
    }
}

