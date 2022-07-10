# Advent-of-Code-2020
My solution for the advent of code 2020, written in Rust

#### My other AoC
- [2021](https://github.com/tmokenc/Advent-of-Code-2021)

## Lessons Learned
- **Day 10 part 2**: [Number of k-combinations for all k](https://en.wikipedia.org/wiki/Combination#Number_of_k-?9!combinations_for_all_k), even tho I didn't use it in my solution, but it's nice to know
- **Day 13 part 2**: [Chinese remainder theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Computation)

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
- **Day 13** - I brute forced the part 2 and ended up waiting 1h34m for the answer, then googled it and found out the [Chinese remainder theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Computation)

## Dependencies
- **humantime** - to display the running time
- **owo-colors** - to print colored result in terminal
- **log** - to debug code
- **fern** - driver for the `log` module
