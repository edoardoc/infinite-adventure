# An infinite Solana adventure

This is a simple, endless exploration and collection game.
It uses a Dictionary of Locations approach for random gameplay generation, implementing a "discovery" mechanism:

<i>When the player moves to a new, unexplored exit from their current location, a new location is generated on the fly</i>

When a new location needs to be created, decides on:
  - A description: simple and evocative (e.g., "a patch of mossy ground," "a cluster of rocks," "a small stream"), it works on a list of descriptive phrases and randomly combines them.
  - Possible exits: Randomly decides which directions will lead to new, unexplored areas.
  Ensures that if an exit from location A to a new location B is created, then location B will eventually (when explored) have an exit back towards A or another part of the existing map.
  (It keeps track of the "parent" location that led to the creation of the new one)
  - Collection items: Randomly determines if any items (mushrooms!) are present in the new location and what type/quantity.


## Build etc.
`anchor build --arch sbf`

## Flow


## Features
- Endless game
- Probability
- Map

## Todo
- Pregenerated map
- Collect method is expensive, a failed collect grows mushrooms randomly in the map
- Three types of mushroom
- Integrate mollusk testing

## Open questions
- How multiple users are handled?
- "Social" aspect of mushroom hunting, together in a solana crowd :)
