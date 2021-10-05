const initialize = () => {
  const solver = new Worker("./worker.js");

  const resultsEl = document.getElementById("results")
  const receive = (event) => {
    results.innerText = event.data
  }

  solver.onmessage = receive

  const dispatch = () => {
    const definitions = document.getElementById("definitions").value
    const job_preferences = document.getElementById("job_preferences").value
    const desired_composition = document.getElementById("desired_composition").value

    solver.postMessage([definitions, desired_composition, job_preferences])
  }

  document.getElementById("activate").addEventListener("click", dispatch, false)
}

initialize();
