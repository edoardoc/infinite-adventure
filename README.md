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

## Example interaction Flow

```bash
~/ yarn dev
yarn run v1.22.22
$ ts-node src/index.ts
Payer Public Key: AvGhvdjzdSmvB6CXUSHmqCAaLiaTJQ6ngdSCz8SH94gW
Running client...
Initializing game...
Initialized game. Transaction: pUcscFSus6sKd25KjdBP3n7GqprYGtrjrPNZfH98J2XQRSCTZznuCTB2FDphuSW284uxZbGHs4ChdCEGty2KVJW

Viewing initial location...
Viewed location. Transaction: 5Y2NCwVGAGLxJ2xsigqA5GHiEpvGUDUMJi11rg5yo4bJ5GV1tjSMuqcS6d6t8peWjDueW39kxLExaNAahMjq4RNr
Current Player Location Index: 0
Game Map Locations: [
  {
    description: 'You find yourself in a peaceful meadow.',
    exits: [ [Object], [Object] ],
    items: [ 'common mushroom' ],
    visited: true
  }
]

Attempting to move north...
Moved north. Transaction: 66girUoyQnCTjDZHrUWr74q9BzPE4QNqNhrv47bob7XD69oLsvVDgq1HGsSdnhgJ6c68TdwQ1BywDTqdycxT4JZf
New Player Location Index: 1
Viewed new location. Transaction: 2rhvLvsLHyTFHLr1nSRWC1AADavJBn862JqMh3Qnpx92HDJYvfi27Nw5DwGS1AVSPptGUay3jNCBLV28USARJjxg
Updated Game Map Locations: [
  {
    description: 'You find yourself in a peaceful meadow.',
    exits: [ [Object], [Object] ],
    items: [ 'common mushroom' ],
    visited: true
  },
  {
    description: 'A whispering forest path.',
    exits: [ [Object], [Object] ],
    items: [],
    visited: true
  }
]
✨  Done in 9.01s.
```

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

## references
-  Fisher-Yates shuffle
   https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
