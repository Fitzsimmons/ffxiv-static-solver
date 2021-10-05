importScripts("./pkg/ffxiv_static_solver.js")

async function init_wasm_in_worker() {
	await wasm_bindgen("./pkg/ffxiv_static_solver_bg.wasm")

	self.onmessage = async (event) => {
		const [definitions, desired_composition, job_preferences] = event.data
		console.time("solve")
		const result = wasm_bindgen.solve(definitions, desired_composition, job_preferences)
		console.timeEnd("solve")
		self.postMessage(result)
	}

	self.postMessage("ready")
}

self.onmessage = (event) => {
	console.log("message received", event.data)
	init_wasm_in_worker();
}


