# Advent of Code 2021

My solutions to Advent of Code 2021.

- Last year: <https://github.com/daniel5151/aoc20>

Goals for 2021:

- Solve the questions (duh)
- Each day should have two commits:
  - The "quick and dirty" commit (rightanswer% speedrun)
  - The "idiomatic Rust" commit (nice error handling, fancy iterators, etc...)
- Solutions should have _reasonable_ (i.e: not strictly the _best_) space and time complexity
- Solutions should terminate reasonably quickly

Some non-goals:

- Scoring super high on the leaderboard
  - Not that I won't _try_ to land somewhere on the leaderboard 😉
  - Only have until Dec 11th. I'm on east-coast time after that, and midnight coding != fast coding

## Running

(Assuming you've put the desired day's input into the `inputs` dir, either manually, or via `run.sh`)

```bash
cargo run --release --features extras -- <day> <question>
```

Tests can be run using the standard `cargo test` flow.

```bash
cargo test -- dayX # only runs tests for the particular day
```

## Running (on the day of)

Manually copying question input? Nahhhh, we can do better than that.

When tests are passing and you're ready for prime-time, skip `cargo` and use the `run.sh` script:

```bash
./run.sh <day> <question>
# e.g: ./run 3 1
```

The harness will automatically download question input if a `cookie.txt` is provided.

`cookie.txt`'s should contain the following string:

```
session=53616c...
```

Getting `cookie.txt` is easy:
- Open Chrome
- Navigate to _any_ day's input URL (e.g: https://adventofcode.com/2021/day/1/input)
- Open the Chrome Network Inspector
- Refresh the URL
- Right click the `input` request, and "copy > copy as cURL"
    - the string should include a `-H 'cookie: <cookie.txt>'` component.

## Q: Why are you using a macro to parse input?

Typing speed!

If I'd used a "parse input" function, I'd need to explicitly specify the return
type (e.g `HashMap<usize, Vec<(usize, usize)>>` or what have you). That's a lot
of keystrokes...

> Okay, so why not just use a closure then? It'll infer the types for you!

Sure... but then error handling would be more annoying, since the final return
value would have to be wrapped with `Ok()`. Plus, unlike the macro, you'd have
to manually copy the closure when it comes time to solve `q2`.

So yeah, the macro approach is a bit weird, but there is method to the madness.
