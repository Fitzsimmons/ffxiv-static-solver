mod utils;
mod job;
mod player;
mod slot;
mod slots;
mod solver;

use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use serde_json::{self, Value};

use job::Job;
use player::Player;
use slots::Slots;
use solver::Solver;

use wasm_bindgen::prelude::*;

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

    let definitions = parse_definitions(definitions).unwrap();
    let players = job_preferences_to_players(job_preferences).unwrap();
    let json_composition: Value = serde_json::from_str(&desired_composition).unwrap();
    let composition = json_composition.as_object().ok_or("Expected composition to be a JSON object").unwrap();
    let slots = Slots::new(composition, &definitions).unwrap();
    let mut solver = Solver::new(slots, players).unwrap();
    let results = solver.solve();

    format!("{}", results[0])
}

fn parse_definitions(raw_definitions: &str) -> Result<HashMap<String, Vec<Job>>, Box<dyn Error>> {
    let mut definitions = HashMap::new();
    let raw_definitions: Value = serde_json::from_str(raw_definitions)?;

    for (name, raw_jobs) in raw_definitions.as_object().ok_or("Expected a JSON object")?.iter() {
        let jobs: Vec<Job> = raw_jobs.as_array().ok_or(format!("Expected jobs for {} to be an array", name))?.iter().map(|raw_job|{
            Job::from_str(raw_job.as_str().unwrap()).unwrap()
        }).collect();

        definitions.insert(name.to_owned(), jobs);
    }

    Ok(definitions)
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
