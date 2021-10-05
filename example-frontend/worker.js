importScripts("./pkg/ffxiv_static_solver.js")

async function init_wasm_in_worker() {
	await wasm_bindgen("./pkg/ffxiv_static_solver_bg.wasm")

	self.onmessage = async (event) => {
		const [definitions, desired_composition, job_preferences] = event.data
		const result = wasm_bindgen.solve(definitions, desired_composition, job_preferences)
		self.postMessage(result)
	}
}

init_wasm_in_worker();
