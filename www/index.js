import * as solver from "ffxiv-static-solver";

const button = document.getElementById("activate")

const solve = () => {
  const definitions = document.getElementById("definitions").value
  const job_preferences = document.getElementById("job_preferences").value
  const desired_composition = document.getElementById("desired_composition").value

  console.log(job_preferences)

  const result = solver.solve(definitions, desired_composition, job_preferences)

  document.getElementById("results").innerText = result
}

document.getElementById("activate").addEventListener("click", solve, false)
