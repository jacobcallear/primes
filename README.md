# Primes

![Cargo tests and lints status](https://github.com/jacobcallear/primes/actions/workflows/tests.yml/badge.svg)

Find prime numbers in a given range.

## Usage

Command line tool that prints prime numbers to standard output. Commands look
like this:

```shell
./primes <start_of_range_to_search> <end_of_range>
```


## Installation

1. Install [rust](https://www.rust-lang.org/tools/install)
2. Clone or [download](https://github.com/jacobcallear/primes/archive/main.zip)
   this repository
3. Compile with the following shell command:

   ```shell
   cargo build --release
   ```

4. You now have the `primes` executable in *./target/release/*

## Examples

- Find all prime numbers between 1 and 10:

  ```shell
  $ ./primes 1 10
  2
  3
  5
  7
  ```

- Find all primes between 1,000,000,000,000,000 and 1,000,000,000,000,200

  ```shell
  $ ./primes 1000000000000000 1000000000000200
  1000000000000037
  1000000000000091
  1000000000000159
  1000000000000187
  ```
