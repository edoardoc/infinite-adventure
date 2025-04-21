# infinite-adventure
An infinite Solana adventure

every step / location is a room, a description get shown (text adventure)

the map of the game could be stored as a variable length array, or a flat simple rectangular X,Y array where elements get disseminated inside
like naval battle
what elements:
Rock, Tree, Lake, Mushroom, etc

It would be nicer if the game map would be stored to create paths, so that some directions are inhibited:

You cannot go right, there is a wall
you cannot proceed, there is a river
There is a lake in front of you
etc.

the gamemap should be randomly generated, and it would be infinite

purpose of the game?



## Features
- Endless game
- Probability
- Map

## Todo
- Pregenerated map
- Collect method is expensive, a failed collect grows mushrooms randomly in the map
- Three types of mushroom

## Open questions
- How multiple users are handled?
- "Social" aspect of mushroom hunting, together in a solana crowd :)
