# Advent of Code 2021

![rust](https://img.shields.io/badge/language-rust-0b7261?style=flat-square&logo=rust)

This is a collection of my personal solutions for [Advent of Code 2021][aoc2021].

For this year, I'm using AoC2021 as an outlet for learning [Rust][rust].

## Benchmarks

```shell
$ cargo aoc bench
```

Solutions and benchmarks are run on a `t6g.medium` [Graviton2][graviton2] instance on AWS EC2.

Variants marked as `original` were my first working solutions when I submitted them
to the AoC event --- all subsequent variants in the same problem are just me trying
out other approaches (most of the time because I learned something new since
my initial solve.)

|      |                          | Variant              |        Part 1 |        Part 2 |
| ---- | :----------------------- | :------------------- | ------------: | ------------: |
| `01` | **Sonar Sweep**          | `original`           |   `4.0161 μs` |   `9.6716 μs` |
|      |                          | `windows_fold`       |   `1.7726 μs` |   `4.3066 μs` |
|      |                          | `windows_fold_edges` |             — |   `1.6559 μs` |
| `02` | **Dive!**                | `original`           |   `1.3414 μs` |   `1.7086 μs` |
| `03` | **Binary Diagnostic**    | `original`           |  `63.0750 μs` | `155.9200 μs` |
| `04` | **Giant Squid**          | `original`           | `179.2700 μs` | `494.3300 μs` |
| `05` | **Hydrothermal Venture** | `original`           |  `13.6720 ms` |  `26.6500 ms` |


---

## Thanks!

The project template uses the recommended setup as described by [`cargo-aoc`][cargo-aoc].

---

[Website][website] &middot; [@techlifemusic][twitter]

[graviton2]: https://aws.amazon.com/ec2/graviton
[aoc2021]: https://adventofcode.com/2021
[rust]: https://rust-lang.org
[cargo-aoc]: https://github.com/gobanos/cargo-aoc
[website]: https://richardneililagan.com
[twitter]: https://twitter.com/techlifemusic
