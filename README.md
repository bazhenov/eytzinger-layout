# The Eytzinger layout bechmarks

Benchmark code I've written for my article [Fast(er) binary search in Rust](https://www.bazhenov.me/posts/faster-binary-search-in-Rust/).

## How to run benchmarks

- standart binary search
  ```console
  $ make -B std.csv
  ```
- the eytzinger layout binary search
  ```console
  $ make eytzinger.csv
  ```
- the branchless eytzinger layout binary search
  ```console
  $ make eytzinger-branchless.csv
  ```
- the branchless eytzinger layout binary search with software memory prefetch
  ```console
  $ make eytzinger-branchless-prefetch.csv
  ```
- generating graph
  ```console
  $ gnuplot plot.gnuplot
  ```
