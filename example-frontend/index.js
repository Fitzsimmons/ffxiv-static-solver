const initialize = async () => {
  const solve = await ffxiv_static_solver.initialize()

  const dispatch = async () => {
    const definitions = document.getElementById("definitions").value
    const job_preferences = document.getElementById("job_preferences").value
    const desired_composition = document.getElementById("desired_composition").value

    const results = await solve(definitions, desired_composition, job_preferences)
    document.getElementById("results").innerText = results
  }

  const button = document.getElementById("activate")

  button.innerHTML = "Solve!"
  button.disabled = false
  button.addEventListener("click", dispatch, false)
}

document.addEventListener("DOMContentLoaded", () => {
  initialize()
});

