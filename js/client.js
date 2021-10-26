ffxiv_static_solver = {};
ffxiv_static_solver.initialize = async () => {
  const solver = new Worker("./ffxiv-static-solver/worker.js");
  solver.postMessage("init")

  const workerReady = new Promise((resolve, _reject) => {
    // the first message we recieve back is letting us know that initialization is complete
    solver.onmessage = (_event) => {
      const solve = (definitions, desired_composition, job_preferences) => {
        var receive = null

        const solutionReady = new Promise((inner_resolve, _inner_reject) => {
          receive = (event) => {
            inner_resolve(event.data)
          }
        })

        solver.onmessage = receive
        solver.postMessage([definitions, desired_composition, job_preferences])

        return solutionReady
      }

      resolve(solve)
    }
  })

  return workerReady
}
