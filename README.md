# FFXIV static composition solver

This is a rust-wasm library for solving the problem of figuring out what job each player should play in their [FFXIV](https://www.finalfantasyxiv.com/) static composition.

## ⚠️ Warning ⚠️

This package is currently an alpha-quality release by an amateur rust programmer. I also don't know very much about wasm either.

* There are no input validations. It is absolutely possible to feed it browser-crashing input at this time.
* Errors that occur in rust are thrown as string javascript exceptions. I have made no effort to make these error messages useful.

See the Contributing section if you're interested in helping with these glaring problems.

## Usage

There's only one function, `solve`. All arguments and the return value are strings in JSON format. A description of each follows. 

```js
import * as solver from "ffxiv-static-solver"
const result = solver.solve(definitions, desired_composition, job_preferences)
```

A small webpack-based proof of concept frontend implementation can be found in the [example-frontend](example-frontend) directory.

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

There could be multiple results, in the case of a tie (see the "How it works" section for more information about how solutions are scored). In the worst-case scenario (all players have identical preferences), the number of results will be the factorial of the number of players. For example, for an 8-player party, the worst case has 8! results, which is 40320. The library currently lacks a way to reject inputs that result in absurdly-sized result sets, so it's up to the client to handle this problem.

## How it works

### Iteration

The `solve` funtion is exhasutive. It accomplishes this via nested loops.

The outer loop iterates over every permutation of how the list of players can be ordered. The number of iterations is equal to the factorial of the number of players. I have found that for 8-player parties, the solution can be found in under a second, even for worst-case inputs.

The inner loop iterates over the list of players, assigning them the best unoccupied slot that can be found based on the order of the jobs provided.

### Scoring

Each potential solution is scored using [Mean squared error](https://en.wikipedia.org/wiki/Mean_squared_error), where "error" is the index of the selected job from the player's preferences. Solutions that do not have the lowest mean squared error are rejected. However, some inputs can result in ties, so all of the results with the lowest score are returned. Mean squared error was selected to attempt to pick a "fair" solution, but other methods of scoring are obviously possible. If you have any suggestions about superior or alternative algorithms, don't hesitate to open an issue on github.

## Known users

So far, the only known public usage of this package is the example implementation, which is obviously lacking. If you would like your site to listed here, open an issue on github.

* http://ffxiv-static-solver-production.s3-website.us-east-2.amazonaws.com/

## Contributing

Contributions welcome! Before starting work on a new feature or API change, check the github issues to see if it's already being worked on, or create your own.

I'm especially interested in improving the API:

* Controlling iteration from javascript: while results are calculated very quickly, I would like to give javascript more control over iteration, so that the browser doesn't lock up. I have not been able to figure out how to store an iterator in a struct in rust, which has prevented me from implementing a tick-based API.
* It's unclear to me if an API that takes (and returns?) `&JsValue` instead of JSON-formatted `&str` would be possible/superior. Any feedback on that would be appreciated.
* There are no input validations. I doubt that infinte loops are possible but it is absolutely possible to feed browser-crashing input
