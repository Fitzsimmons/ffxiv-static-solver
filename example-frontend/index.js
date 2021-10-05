const initialize = () => {
  const solver = new Worker("./worker.js");
  solver.postMessage("init")

  const workerReady = new Promise((resolve, reject) => {
    solver.onmessage = (event) => {
      resolve(event.data)
    }
  })

  const resultsEl = document.getElementById("results")
  const receive = (event) => {
    results.innerText = event.data
  }

  const dispatch = () => {
    const definitions = document.getElementById("definitions").value
    const job_preferences = document.getElementById("job_preferences").value
    const desired_composition = document.getElementById("desired_composition").value

    solver.postMessage([definitions, desired_composition, job_preferences])
  }

  workerReady.then(() => {
    solver.onmessage = receive
  })

  return workerReady
}

initialize().then(() => {
  const button = document.getElementById("activate")

  button.innerHTML = "Solve!"
  button.disabled = false
  button.addEventListener("click", dispatch, false)
})
