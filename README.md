## Game of Life

### Installation
- Linux
  - Clone this repository
  - `sudo apt install libsdl2-dev`
  - `sudo apt install libasound2-dev`
  - Run `target/release/game_of_life`
___
### Rules
1) Each cell can be either alive or dead
2) Cell survives if it has 2 or 3 neighbours
3) Cell comes lo life if it has 3 neighbours
4) All other cells die

___
### Controls
- Point and _click_ on the cell to make it __come to life__ (if it's dead). Only works when the game is __not__ running
- Point and _click_ on the cell to __kill__ it (if it's alive). Only works when the game is __not__ running
- Press _spacebar_ to __run__ the game
- Press _spacebar_ to __pause__ the game
- Press _R_ to __reset__ the game
- Press _Q_ or _Esc_ to __quit__ the game
