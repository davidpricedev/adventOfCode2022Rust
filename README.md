# adventOfCode2022Rust

This repository contains my solutions for many of the puzzles in the [advent of code 2022](https://adventofcode.com/2022).

As I usually do, I've decided to learn a new language while solving these puzzles. This year the new language is Rust.

## Some notes about rust

At first glance, rust appears to borrow a lot from python

- same type syntax
- same preference for snake_case
- same self argument for all instance methods
- similar folder structure concepts (though thankfully no __init__.rs files needed!)
- even some libraries (itertools) have the same name as python

## Reaction

Rust may use a similar struct + functions concept as golang does, but couldn't be more different.
You can learn 90% of golang in an afternoon, but I think you'd only learn 10% of rust in that afternoon.
I suppose that is to be expected with a language that has strong guarantees, but also tries to be somewhat high-level.

### returns from functions

I like the idea of the last line in the function implicitly returning.
I'm not so sure about the fact that only works when skipping the semi-colon on the last line.
That seems like a glitch more than something that was actually designed into the language.

### compiler errors

Compiler errors seem to be generally fantastic.
They tell you exactly what the problem is and sometimes even how to fix it.
I'm seeing so many compiler errors as I learn the basics here
