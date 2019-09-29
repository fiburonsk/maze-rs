# Maze

## Build

* Clone the repository and enter the base folder
* `cargo run`

## Usage

```bash
Maze generation and solving application.

USAGE:
    maze [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --build-speed <build>    Set the build speed. Lower is faster. A value of zero will not show building the maze.
                                 [default: 0]
    -h, --height <height>        Height of maze. Use odd numbers for a wall on the border. [default: 11]
    -i, --image <image>          Save the solved maze as a .png image.  Provide the path and filename to save an image.
    -m, --method <method>        Maze generation method. [default: backtracker]  [possible values: backtracker, prims,
                                 prims-backtracker]
    -r, --seed <seed>            Seed used by the random number generator to build the maze. [default: 1]
    -s, --solve-speed <solve>    Set the solve speed. Lower is faster. A value of zero will not show solving the maze.
                                 [default: 0]
    -w, --width <width>          Width of maze, Use odd numbers for a wall on the border. [default: 11]
```
