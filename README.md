# FFXIV static composition solver

We have a bunch of people and need to figure out what role they'll play in our static. How do we decide?

## Role categorization

First, we define the roles as they are in FFXIV (not exhaustive):

```json
{
  "ranged dps": ["BRD", "MCH", "DNC", "BLM", "RDM", "SMN"],
  "melee dps": ["DRG", "MNK", "SAM", "NIN", "RPR"],
  "tank": ["GNB", "PLD", "WAR", "DRK"],
  "barrier healer": ["SCH", "SGE"],
  "pure healer": ["WHM", "AST"],
  "mage": ["BLM", "RDM", "SMN"],
  "ranged physical": ["DNC", "BRD", "MCH"]
}
```

## Desired composition

We also need to specify our desired composition as pairs of roles to how many slots of that role to fill:

```json
{
  "ranged dps": 2,
  "melee dps": 2,
  "tank": 2,
  "barrier healer": 1,
  "pure healer": 1,
}
```

## Player preferences

Each player lists the jobs they're willing and able to play, ranked in order of preference. This can be compiled into a preferences data structure like so:

```json
{
  "Yorvo Hawke": ["DRG", "GNB"],
  "Squidgy Bunny": ["NIN", "SMN", "WHM", "PLD"],
  "Renfleur Orinoux": ["DRK", "SAM"],
  "Zelle Tamjin": ["PLD", "BLM"],
  "Era Dere": ["WHM", "DNC"],
  "Brando Id": ["SCH"],
  "Alleriana Valyrian": ["RDM", "BLM"],
  "Reye Fenris": ["BRD", "DRG"],
}
```

## Desired output

### Phase 1

To start, we're looking for any valid output that fills each desired role, e.g.

```json
{
  "Yorvo Hawke": "DRG",
  "Squidgy Bunny": "NIN",
  "Renfleur Orinoux": "DRK",
  "Zelle Tamjin": "PLD",
  "Era Dere": "WHM",
  "Brando Id": "SCH",
  "Alleriana Valyrian": "RDM",
  "Reye Fenris": "BRD",
}
```

### Phase 2

What determines the "best" fit? We can tune our algorithm to prioritize job assignments in different ways.

For example, is it better to have 7 people with their 1st choice and 1 person with their 8th choice, or 8 people with their 2nd choice? Let's iterate on the algorithm to see what we can come up with.
