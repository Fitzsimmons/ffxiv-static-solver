import * as solver from "ffxiv-static-solver";

const solve = () => {
  const definitions = document.getElementById("definitions").value
  const job_preferences = document.getElementById("job_preferences").value
  const desired_composition = document.getElementById("desired_composition").value

  console.time("solve")
  const result = solver.solve(definitions, desired_composition, job_preferences)
  console.timeEnd("solve")

  document.getElementById("results").innerText = result
}

document.getElementById("activate").addEventListener("click", solve, false)
