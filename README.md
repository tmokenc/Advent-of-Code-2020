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
- **Day 10** - Added `0` and `last + 3` to the list and sort it in the parsing phase

## Remarkable Memories
- **Day 7** - Ahh yes, the first recursion of this year challenge, I was able to implement a non-recursive function in the first part, but not the second part
- **Day 10** - spent an entire day for the part 2
- **Day 12** - I really enjoy this one because it feels like making a game logic

## Dependencies
- **humantime** - to display the running time
- **owo-colors** - to print colored result in terminal
- **log** - to debug code
- **fern** - driver for the `log` module
