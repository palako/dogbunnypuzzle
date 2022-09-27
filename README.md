# dogbunnypuzzle
Solution to the http://www.dogbunnypuzzle.com/ game

The first commit is to stop after finding the first solution on a recursive depth-first approach.
It marks every state that has been added to the list of states pending to be visisted (boolean), 
and it considers states with either bunny in a node equivalent.

The second commit stores the number of steps needed to reach each state instead of a boolean to mark if a state has been
marked as pending to be visited, and if a faster path is found, it will add it again with the shorter path.
The algorithm won't stop after finding a solution, and thus continues finding eventually shorter and shorter solutions until
arriving at the shortest at 26 steps.

This was just a short fun project to come back to sake the cobwebs in Rust which I had not touched in a good while.

Everything is very hardcoded, since at the time of solving there is only one variant of the game but the author mentioned on twitter that was working on making more. Once something else if published I'll probably come back to this to be able to parametrise the graph, initial and end states, conditions, etc..
