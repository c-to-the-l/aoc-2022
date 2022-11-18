# Advent of code 2022

Advent of code 2022 solutions, as and when they are ready.

## Usage

### Session cookie

This repository requires the environment variable `AOC_SESSION` to be set, to fetch your input data. 

This must be set to the contents of your AoC session cookie.

You can find this using your browser's developer console, for chrome this is the region hidden by the red box in the below image:

![image](https://user-images.githubusercontent.com/39165068/202818944-c5430798-4b7a-4499-8013-b9338d2ee731.png)

You can either provide the variable directly...

```
AOC_SESSION=abc123xyz cargo run -- ...
```

Or make a `.env` file in the root of the project, that looks something like this:

```
AOC_SESSION=abc123obviouslyfakxyz
```

### Command line usage

```
Usage: aoc-2022 [PROBLEMS]...

Arguments:
  [PROBLEMS]...  One or more problem numbers or hyphenated ranges of the form "x-y"

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

Example command line usage:

```
cargo run -- 1 3 5-8
```

This will evaluate problems 1, 3, 5, 6, 7, 8 in that order.

Running the program with no arguments (`cargo run`) will run all problems.
