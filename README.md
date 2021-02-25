# Primes

![Cargo tests and lints status](https://github.com/jacobcallear/primes/actions/workflows/tests.yml/badge.svg)

Find prime numbers in a given range.

## Usage

Command line tool that prints prime numbers to standard output. Commands look
like this:

```shell
./primes <start_of_range_to_search> <end_of_range>
```

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
