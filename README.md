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
Accounts already initialized.
Current Player Location Index: 1
Game Map: [
  {
    "description": "You find yourself in a peaceful meadow.",
    "exits": [
      {
        "direction": "north",
        "targetIndex": 1
      },
      {
        "direction": "east",
        "targetIndex": null
      }
    ],
    "items": [
      "common mushroom"
    ],
    "visited": true
  },
  {
    "description": "A whispering forest path.",
    "exits": [
      {
        "direction": "west",
        "targetIndex": null
      },
      {
        "direction": "east",
        "targetIndex": null
      }
    ],
    "items": [],
    "visited": true
  }
]

Viewing initial location...
Viewed location. Transaction: 3epKxzi93MP86hGJeLx8PV3yzEMK9xxPg2uwVJmdPLPp7atbrEMouiK9TEC9p5vmJPrYrEW7YdmGMFakU6ogzx7P
Current Player Location Index: 1
Game Map: [
  {
    "description": "You find yourself in a peaceful meadow.",
    "exits": [
      {
        "direction": "north",
        "targetIndex": 1
      },
      {
        "direction": "east",
        "targetIndex": null
      }
    ],
    "items": [
      "common mushroom"
    ],
    "visited": true
  },
  {
    "description": "A whispering forest path.",
    "exits": [
      {
        "direction": "west",
        "targetIndex": null
      },
      {
        "direction": "east",
        "targetIndex": null
      }
    ],
    "items": [],
    "visited": true
  }
]

Enter direction to move (north, south, east, west) or "stop" to exit: west

Attempting to move west...
Moved west. Transaction: 4rMwBuxGH9HYNHRxDdj3QPLLNQGvPHP42T3KZDGQwGRGH9KHbbUWhUi8SrDQUqj6LZkrNJPrnSkbbDNUdwqhrmYe
New Player Location Index: 2
Viewed new location. Transaction: 26YLmankfcaVibAANKnET9VDSAenVGXs5i6R55MHPBV7gZBRnLMVUMiNRr7RgvTt8sbdYLyswadZ1eUzbRfk23mv
Updated Game Map: [
  {
    "description": "You find yourself in a peaceful meadow.",
    "exits": [
      {
        "direction": "north",
        "targetIndex": 1
      },
      {
        "direction": "east",
        "targetIndex": null
      }
    ],
    "items": [
      "common mushroom"
    ],
    "visited": true
  },
  {
    "description": "A whispering forest path.",
    "exits": [
      {
        "direction": "west",
        "targetIndex": 2
      },
      {
        "direction": "east",
        "targetIndex": null
      }
    ],
    "items": [],
    "visited": true
  },
  {
    "description": "A whispering forest path.",
    "exits": [
      {
        "direction": "south",
        "targetIndex": null
      },
      {
        "direction": "north",
        "targetIndex": null
      }
    ],
    "items": [],
    "visited": true
  }
]

Enter direction to move (north, south, east, west) or "stop" to exit: south

Attempting to move south...
Moved south. Transaction: 2RxSZJoahYUci6NafxtwaVnbJ3vX4QRv3Jy1qKFAwVrb6Kv7F6XTgX6zGDTa8EXQ6rUzSAAqETJtknWMau9ue83D
New Player Location Index: 3
Viewed new location. Transaction: 2AzpS2Qa9vezCRZjXy2zpV4fwR7cgzNYh4cD56y3i2FFTBnYUn9bJXNtdXYpamUi3WJ4ZTvBV16WVpf4RmXWNiwm
Updated Game Map: [
  {
    "description": "You find yourself in a peaceful meadow.",
    "exits": [
      {
        "direction": "north",
        "targetIndex": 1
      },
      {
        "direction": "east",
        "targetIndex": null
      }
    ],
    "items": [
      "common mushroom"
    ],
    "visited": true
  },
  {
    "description": "A whispering forest path.",
    "exits": [
      {
        "direction": "west",
        "targetIndex": 2
      },
      {
        "direction": "east",
        "targetIndex": null
      }
    ],
    "items": [],
    "visited": true
  },
  {
    "description": "A whispering forest path.",
    "exits": [
      {
        "direction": "south",
        "targetIndex": 3
      },
      {
        "direction": "north",
        "targetIndex": null
      }
    ],
    "items": [],
    "visited": true
  },
  {
    "description": "A quiet clearing.",
    "exits": [],
    "items": [],
    "visited": true
  }
]

Enter direction to move (north, south, east, west) or "stop" to exit: stop
Exiting movement loop...
✨  Done in 64.05s.
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
