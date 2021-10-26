importScripts("./ffxiv_static_solver.js")

async function init_wasm_in_worker() {
	await wasm_bindgen("./ffxiv_static_solver_bg.wasm")

	self.onmessage = (event) => {
		const [definitions, desired_composition, job_preferences] = event.data
		const result = wasm_bindgen.solve(definitions, desired_composition, job_preferences)
		self.postMessage(result)
	}

	self.postMessage("ready")
}

// assume the first message is a request to initialize
self.onmessage = () => {
	init_wasm_in_worker();
}
