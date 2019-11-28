# pretty-ls

The ls command with a prettier output.


## Goals

- [x] Add filetype specific sybols
- [ ] Add colors
- [ ] Add options for
  - [ ] sorting
  - [ ] dates, privileges
- [ ] Git status


## Benchmark

Executed for a small folder. Compiled with the `--release` option. Measured with `time`.
```
| program | total CPU time |
| ------- | -------------- |
| ls      |       0.005 ms |
| pls     |       0.007 ms |
```
