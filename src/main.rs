mod utils;
mod job;
mod player;
mod slot;
mod slots;
mod solver;

use job::Job;
use player::Player;
use slots::Slots;
use solver::Solver;

use std::str::FromStr;
use std::error::Error;
use serde_json::Value;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
	let definitions = r#"
	{
	  "ranged dps": ["BRD", "MCH", "DNC", "BLM", "RDM", "SMN"],
	  "melee dps": ["DRG", "MNK", "SAM", "NIN", "RPR"],
	  "tank": ["GNB", "PLD", "WAR", "DRK"],
	  "healer": ["WHM", "SCH", "AST", "SGE"],
	  "barrier healer": ["SCH", "SGE"],
	  "pure healer": ["WHM", "AST"],
	  "mage": ["BLM", "RDM", "SMN"],
	  "ranged physical": ["DNC", "BRD", "MCH"]
	}
	"#;

	let desired_composition = r#"
	{
	  "ranged dps": 2,
	  "melee dps": 2,
	  "tank": 2,
	  "barrier healer": 1,
	  "pure healer": 1
	}
	"#;

	let job_preferences = r#"
	{
	  "Yorvo Hawke": ["DRG", "GNB"],
	  "Squidgy Bunny": ["NIN", "SMN", "WHM", "PLD"],
	  "Renfleur Orinoux": ["DRK", "SAM"],
	  "Zelle Tamjin": ["PLD", "BLM"],
	  "Era Dere": ["WHM", "SCH", "DNC"],
	  "Brando Id": ["AST"],
	  "Alleriana Valyrian": ["RDM", "BLM"],
	  "Reye Fenris": ["BRD", "DRG"]
	}
	"#;

	let definitions = parse_definitions(definitions)?;
	let players = job_preferences_to_players(job_preferences)?;
	let json_composition: Value = serde_json::from_str(&desired_composition)?;
	let composition = json_composition.as_object().ok_or("Expected composition to be a JSON object")?;
	let slots = Slots::new(composition, &definitions)?;
	let mut solver = Solver::new(slots, players)?;
	let results = solver.solve();

	eprintln!("{} result{} found", results.len(), if results.len() != 1 {"s"} else {""});
	println!("{}", serde_json::to_string(&results)?);

	Ok(())
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
