# FFXIV static composition solver

This is a rust-wasm library for solving the problem of figuring out what job each player should play in their [FFXIV](https://www.finalfantasyxiv.com/) static composition.

## ⚠️ Warning ⚠️

This package is currently an alpha-quality release by an amateur rust programmer. I also don't know very much about wasm either.

* There are no input validations. I imagine it is entirely possible to feed it browser-crashing input at this time.
* Errors that occur in rust are thrown as string javascript exceptions. I have made no effort to make these error messages useful.

See the Contributing section if you're interested in helping with these glaring problems.

## Usage

This library is distributed as a wasm module that's loaded and executed inside a Web Worker so that the computation can take place in the background. There's also a small wrapper ([client.js](js/client.js)) that abstracts away as much of these implementation details as possible.

⚠️ The input and output from the client function are JSON-formatted strings; plain javascript objects are not supported at this time. See the Contributing section for more information. ⚠️

### Installing and sourcing the package

Download the latest release from the [releases page](https://github.com/Fitzsimmons/ffxiv-static-solver/releases), and unzip into your project's public web directory.

Include the client script in your html:

```html
...
<!-- Adds `ffxiv_static_solver` as a global variable -->
<script src="./ffxiv-static-solver/client.js"></script>

<!-- 
Use `initialize` to set up the background worker.
It returns a function that can be used to query the solver.
-->
<script>
  document.addEventListener("DOMContentLoaded", async () => {
    const solve = await ffxiv_static_solver.initialize()

    // ...

    const results = await solve(definitions, desired_composition, job_preferences)
  });
</script>
```

See the [example-frontend/index.html](example-frontend/index.html) for a more complete example.

### Role definitions

First, we define the roles an object with an arbitrary string as a key and a list of jobs that can fulfill that role as the value. For example:

```json
{
  "ranged dps": ["BRD", "MCH", "DNC", "BLM", "RDM", "SMN"],
  "melee dps": ["DRG", "MNK", "SAM", "NIN", "RPR"],
  "tank": ["GNB", "PLD", "WAR", "DRK"],
  "healer": ["WHM", "SCH", "AST", "SGE"],
  "barrier healer": ["SCH", "SGE"],
  "pure healer": ["WHM", "AST"],
  "mage": ["BLM", "RDM", "SMN"],
  "ranged physical": ["DNC", "BRD", "MCH"]
}
```

### Desired composition

We also need to specify our desired composition as pairs of roles to how many slots of that role to fill. The keys for this object need to match the definitions, and the value is a number indicating how many slots of that role that need to be filled. For example:

```json
{
  "ranged dps": 2,
  "melee dps": 2,
  "tank": 2,
  "barrier healer": 1,
  "pure healer": 1
}
```

### Player preferences

Finally, we provide the player data. Each key in this object is the player name, and the value is a list of jobs, in order of preference, that the player wants to play.

```json
{
  "Yorvo Hawke": ["DRG", "GNB"],
  "Squidgy Bunny": ["NIN", "SMN", "WHM", "PLD"],
  "Renfleur Orinoux": ["DRK", "SAM"],
  "Zelle Tamjin": ["PLD", "BLM"],
  "Era Dere": ["WHM", "SCH", "DNC"],
  "Brando Id": ["AST"],
  "Alleriana Valyrian": ["RDM", "BLM"],
  "Reye Fenris": ["BRD", "DRG"]
}
```

### Results

The results are an array of flat objects, where the keys are the player names and the values are the assigned job. 

```json
[
  {
    "Era Dere": "SCH",
    "Squidgy Bunny": "NIN",
    "Yorvo Hawke": "DRG",
    "Brando Id": "AST",
    "Alleriana Valyrian": "RDM",
    "Reye Fenris": "BRD",
    "Renfleur Orinoux": "DRK",
    "Zelle Tamjin": "PLD"
  }
]
```

There could be multiple results, in the case of a tie (see the "How it works" section for more information about how solutions are scored). In the worst-case scenario (all players have all jobs and in the same order), the number of results will be the factorial of the number of players. For example, for an 8-player party, the worst case has 8! results, which is 40320. The library currently lacks a way to reject inputs that result in absurdly-sized result sets, so it's up to the client to handle this problem.

## How it works

### Iteration

The `solve` funtion is exhasutive. It accomplishes this via nested loops.

The outer loop iterates over every permutation of how the list of players can be ordered. The number of iterations is equal to the factorial of the number of players.

The inner loop iterates over the list of players, assigning them the best unoccupied slot that can be found based on the order of the jobs provided.

I have found that for 8-player parties, the solution can be found in under a second on average hardware, even for worst-case inputs.

### Scoring

Each potential solution is scored using [Mean squared error](https://en.wikipedia.org/wiki/Mean_squared_error), where "error" is the index of the selected job from the player's preferences (i.e. higher preference = lower error). Solutions that do not have the lowest mean squared error are rejected. However, some inputs can result in ties, so all of the results with the lowest score are returned. Mean squared error was selected in an attempt to pick a "fair" solution, but other methods of scoring are obviously possible. If you have any suggestions about superior or alternative algorithms, don't hesitate to open an issue on github.

## Known users

So far, the only known public usage of this package is the example implementation, which is obviously lacking. If you would like your site to be listed here, open an issue on github.

* http://ffxiv-static-solver-production.s3-website.us-east-2.amazonaws.com/

## Contributing

Contributions welcome! Before starting work on a new feature or API change, check the github issues to see if it's already being worked on, or create your own.

Top items that I would appreicate help with:

* The wasm module will throw javascript exceptions if it encounters errors but they're completely unhandled by the client javascript.
* It's unclear to me if an API that takes (and returns?) plain javascript objects instead of JSON-formatted `&str` would be possible/superior. Any feedback on that would be appreciated.
* There are no input validations. Users can probably crash their browser with malformed or comically large input.
* Javascript packaging: I'm aware that old fashioned script-tag imports for javascript that put objects into the global scope are not modern. I've done this because I'm not sure how to ship the wasm-in-web-worker code as anything else, since I'm just following what the [wasm-bindgen examples](https://github.com/rustwasm/wasm-bindgen/tree/d4b21e7d66638f9ef46396f6179f1cde7b3fa352/examples/wasm-in-web-worker) do for that. Apparently there is improving support for loading web workers as modules, but firefox lacks support for it yet. I'm also not sure how that interacts with wasm, if at all.

