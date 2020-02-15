# Maze

## Build

* Clone the repository and enter the base folder
* `cargo run`

## Usage

```bash
Usage: ./maze [-r <seed>] [-h <height>] [-w <width>] [-s <solve-speed>] [-b <build-speed>] [-m <method>] [-i <image>]

Maze solving application.

Options:
  -r, --seed        seed used to build the maze [default: 1]
  -h, --height      height [default: 11]
  -w, --width       width [default: 11]
  -s, --solve-speed solve speed use 0 to hide [default: 0]
  -b, --build-speed build speed use 0 to hide [default: 0]
  -m, --method      solving method: [backtracker, prims, prims-backtracker]
                    [default: backtracker]
  -i, --image       save result to image
  --help            display usage information
```
