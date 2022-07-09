# Advent-of-Code-2020
My solution for the advent of code 2020, written in Rust

#### My other AoC
- [2021](https://github.com/tmokenc/Advent-of-Code-2021)

## Lessons Learned
Nothing yet

## Error Handle
**NOPE!!!** `unwrap()` everything everywhere

## Use
To run all of the advent of code solutions
```sh
cargo run --release
```
To run only a specific day (replace {DAY_NUMBER} with the number of that day)
```sh
cargo run --release -- {DAY_NUMBER}
```

#### Note
- **Day 9** - Does not have a example run, because its input is different from the real input
Nothing yet

## Remarkable Memories
- **Day 7** - Ahh yes, the first recursion of this year challenge, I was able to implement a non-recursive function in the first part, but not the second part


## Dependencies
- **humantime** - to display the running time
- **owo-colors** - to print colored result in terminal
- **log** - to debug code
- **fern** - driver for the `log` module
