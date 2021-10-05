let ffxiv_static_solver;
ffxiv_static_solver.initialize = () => {
  const solver = new Worker("./worker.js");
  solver.postMessage("init")

  // rewrite as async function?
  const workerReady = new Promise((resolve, reject) => {
    // the first message we recieve back is letting us know that initialization is complete
    solver.onmessage = (event) => {
      const solve = (definitions, desired_composition, job_preferences) => {

        const solutionReady = new Promise((inner_resolve, inner_reject) => {
          receive = (event) => {
            inner_resolve(event.data)
          }
        })

        solver.onmessage = receive
        solver.postMessage([definitions, desired_composition, job_preferences])

        return solutionReady
      }

      // we resolve this promise with a function that can dispatch a message to the worker and returns a promise that resolves with the solution
      resolve(solve)
    }
  })

  return workerReady
}
