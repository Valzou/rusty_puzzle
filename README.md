# Rusty Puzzle

Rusty Puzzle is a puzzle game developed in Rust.

## Purpose
A playable puzzle with a graphic interface

## Install
```sh
    $ git clone git@github.come:vch9/rusty-puzzle.git
    $ cd rusty-puzzle
    $ make 
```

## What is this game ?

It's a puzzle game where the rectangles colors needs to match lines colors

## Lines colors:
    * Red:
      You need more red rectangles around this line than blue rectangles
    * Blue:
      You need more blue rectanglese around this line than red rectangles
    * Magenta:
      You need the same numbers of blue and red rectangles around this line

## How to play ?
Either you compile the project with the Makefile and run this:

```sh
    $ make
    $ ./puzzle (n') (width') (height') 
```

Or with cargo:

```sh
    $ cargo run (n') (width') (heigh')
```


2^n is the number of lines in the puzzle, width * height are the window's dimension.
