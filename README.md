# Advent of Code 2021

![rust](https://img.shields.io/badge/language-rust-0b7261?style=flat-square&logo=rust)

This is a collection of my personal solutions for [Advent of Code 2021][aoc2021].

For this year, I'm using AoC2021 as an outlet for learning [Rust][rust].

## Benchmarks

```shell
$ cargo aoc bench
```

Solutions and benchmarks are run on a `t6g.medium` [Graviton2][graviton2] instance on AWS EC2.

| | | Method | Part 1 | Part 2 |
| --- | :-- | :-- | :-- | :-- |
| `01` | **Sonar Sweep** | Original | `4.0161 μs` | `9.6716 μs` |
| | | `windows` + `fold` | `1.7726 μs` | `4.3066 μs` |


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
