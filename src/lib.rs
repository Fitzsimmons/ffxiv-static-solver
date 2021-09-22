mod utils;
mod job;
mod player;
mod slot;
mod slots;
mod solver;

use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use once_cell::sync::OnceCell;
use serde_json::{self, Value};

use job::Job;
use player::Player;
use slots::Slots;
use solver::Solver;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello() {
    log("hello");
}

#[wasm_bindgen]
pub fn solve(definitions: &str, desired_composition: &str, job_preferences: &str) -> String {
    // log(definitions);
    // log(desired_composition);
    // log(job_preferences);

    utils::set_panic_hook();
    load_definitions(definitions).unwrap();

    let players = job_preferences_to_players(job_preferences).unwrap();
    let json_composition: Value = serde_json::from_str(&desired_composition).unwrap();
    let slots = Slots::new(json_composition.as_object().ok_or("Expected composition to be a JSON object").unwrap()).unwrap();
    let mut solver = Solver::new(slots, players).unwrap();
    let results = solver.solve();

    format!("{}", results[0])
    // format!("{}", "worked")
}

pub static DEFINITIONS: OnceCell<HashMap<String, Vec<Job>>> = OnceCell::new();

fn load_definitions(raw_definitions: &str) -> Result<(), Box<dyn Error>> {
    let mut classifications = HashMap::new();
    let raw_classifications: Value = serde_json::from_str(raw_definitions)?;

    for (name, raw_jobs) in raw_classifications.as_object().ok_or("Expected a JSON object")?.iter() {
        let jobs: Vec<Job> = raw_jobs.as_array().ok_or(format!("Expected jobs for {} to be an array", name))?.iter().map(|raw_job|{
            Job::from_str(raw_job.as_str().unwrap()).unwrap()
        }).collect();

        classifications.insert(name.to_owned(), jobs);
    }

    DEFINITIONS.set(classifications).unwrap();

    Ok(())
}

fn job_preferences_to_players(raw_input: &str) -> Result<Vec<Player>, Box<dyn Error>> {
    let mut players = Vec::new();
    let preferences: Value = serde_json::from_str(&raw_input)?;

    for (name, raw_jobs) in preferences.as_object().ok_or("Expected a JSON object")?.iter() {
        let jobs: Vec<Job> = raw_jobs.as_array().ok_or(format!("Expected jobs for {} to be an array", name))?.iter().map(|raw_job|
            Job::from_str(raw_job.as_str().unwrap()).unwrap()
        ).collect();
        let player = Player { name: name.to_string(), jobs };
        players.push(player);
    }

    return Ok(players);
}
