# Introduction

The purpose of this repo is to brush up on my data structures and algorithms knowledge. To this end it isn't enough to just solve leetcode questions but I will be using criterion (a micro-benchmarking utility) to optimise the solutions.

Most of the optimisations implemented here will come from my reading of [The Rust Performance Book](https://nnethercote.github.io/perf-book/) along with various gitbub repos, blogs and YouTube videos.

## Utility Cargo Subcommand

`cargo-problem` has a Cargo subcommand that can insalled by running `cargo install --path .` inside the cargo-problem directory. Then running:

`cargo problem folder_name problem_name`

Will create a `problem_name.rs` at `src/problems/folder_name` and append the `mod.rs` inside that folder.

## Hashmap and HashSet

Looking at `contains_duplicates.rs` and `two_sum` I made use of the standard libraries HashSet and HashMap structures, however they make use of the `SipHash 1-3` hashing algorithm, which is a general purpose, denial-of-service resistant hashing function. This isn't required when the inputs are known - which is the case in many scenarios. Looking at the results:

```shell
contains_duplicate/stdlib/large_no_dups
    time:   [92.110 µs 92.813 µs 93.605 µs]

contains_duplicate/ahash/large_no_dups
    time:   [70.889 µs 71.135 µs 71.402 µs]

contains_duplicate/stdlib/large_with_dups
    time:   [82.425 µs 82.911 µs 83.554 µs]

contains_duplicate/ahash/large_with_dups
    time:   [56.410 µs 56.640 µs 56.890 µs]

two_sum/stdlib/large
    time:   [1.6487 µs 1.6559 µs 1.6646 µs]

two_sum/ahash/large
    time:   [1.3897 µs 1.3920 µs 1.3946 µs]
```

##
