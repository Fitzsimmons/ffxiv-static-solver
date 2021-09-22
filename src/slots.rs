use std::error::Error;
use std::fmt;

use serde_json::Value;

use crate::Job;
use crate::Player;
use crate::slot::Slot;

#[derive(Clone, PartialEq, Eq)]
pub struct Slots {
    pub slots: Vec<Slot>,
    claimed_jobs: Vec<Job>,
}

impl Slots {
    pub fn new(composition: &serde_json::Map<String, Value>) -> Result<Slots, Box<dyn Error>> {
        let mut slots = Vec::new();

        for (role, raw_count) in composition.iter() {
            let count = raw_count.as_u64().ok_or(format!("Expected value for {} to be a number", role))?;
            for _ in 0..count {
                slots.push(Slot::new(role.clone()))
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
}

impl fmt::Display for Slots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for slot in &self.slots {
            write!(f, "{}\n", slot)?;
        }
        Ok(())
    }
}
