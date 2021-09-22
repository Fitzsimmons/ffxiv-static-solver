use std::error::Error;

use crate::Slots;
use crate::Player;

use permute::permutations_of;

pub struct Solver {
	slots: Slots,
	players: Vec<Player>,
	// player_order_permutations_iterator: Box<Iterator<Item = impl Iterator<Item = &String>>>
}

impl Solver {
	pub fn new(slots: Slots, players: Vec<Player>) -> Result<Solver, Box<dyn Error>> {

		Ok(Solver{slots, players})
	}

	pub fn solve(&mut self) -> Vec<Slots> {
		let mut potential_solutions = Vec::new();
		let mut lowest_mean_squared_error: f64 = f64::MAX;

		let player_permutations = permutations_of(&self.players);

		let mut workspace_slots = self.slots.clone();

		for permutation in player_permutations {
			workspace_slots.unassign();
			if self.calculate_solution_for_permutation(&mut workspace_slots, permutation).is_err() { continue; }

			match workspace_slots.mean_squared_error() {
				Some(error) => {
					if error == lowest_mean_squared_error {
						potential_solutions.push(workspace_slots.clone());
						continue;
					}

					if error < lowest_mean_squared_error {
						crate::log(&format!("Found new solution:\n {}", workspace_slots));
						lowest_mean_squared_error = error;
						potential_solutions.clear();
						if potential_solutions.iter().find(|p| p == &&workspace_slots).is_none() {
							potential_solutions.push(workspace_slots.clone());
						}
						continue;
					}
				}
				None => { continue; }
			}
		}

		return potential_solutions;
	}

	pub fn calculate_solution_for_permutation<'a>(&self, slots_buf: &mut Slots, permutation: impl Iterator<Item=&'a Player>) -> Result<(), Box<dyn Error>> {
		for player in permutation {
			// slots_buf.assign(player)?

			// match slots_buf.assign(player) {
			// 	Ok(_) => {},
			// 	Err(_) => {
			// 		crate::log(&format!("Could not find slot for {}. Current slot layout:\n{}", player.name, slots_buf));
			// 	}
			// }

			slots_buf.assign(player)?
		}
		Ok(())
	}
}
