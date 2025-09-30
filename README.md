# README

Hyperscrambler (HSCR) is a tool for generating scrambles for [Hyperspeedcube](https://ajfarkas.dev/projects/hyperspeedcube/). It allows for practicing particular steps of a solve, which is otherwise inconvenient.

## Usage

Call Hyperscrambler with a path to a scramble definition, and optionally, a output path.

```cargo run --release -- --input definitions/right_block.hscr --output temp.hsc```

It will print (or write to the output path) the generated scramble.

## Scramble definitions

A `.hscr` scramble definition consists of

- the size and dimension of the puzzle,
- the number of times to sample the generators,
- a prefix, which will be prepended to the scramble,
- a postfix, which will be appended to the scramble,
- a newline separated list of generators.

The prefix, postfix, and generators must be a space separated list of HSC twists. Empty lines and everything following `//` on a line will be ignored. Consider changing the prefix to rotate the puzzle to your desired solving orientation.

## TODO

If you want some feature added, ping me in the hypercubes discord server.

Things I may add:

- autocopy
- web
- gui
- github actions releases
- changelog
- add visible_pieces to .hscr
- make .hscr sections optional
