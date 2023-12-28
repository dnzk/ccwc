## CCWC

CCWC is an implementation of [this challenge](https://codingchallenges.fyi/challenges/challenge-wc/). It's Coding Challenge's `wc` tool (thus CCWC), written in Rust. `wc` is a tool that when given a file path, will return the lines, characters, and bytes count of the file. Currently it supports all the options and any permutations of them:

* `-c` to show bytes count.
* `-l` to show lines count.
* `-w` to show words count.
* `-m` to show characters count.

It shows the report rather differently, but it can be configured in `reports.rs`. It also supports piping:

```
cat file_name.txt | ccwc -l
```

## Notes

Currently it's dependent on the argument's number and position. This means that when you supply the filename before the options, it will fail.
