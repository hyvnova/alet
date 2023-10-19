# Alet

A simple and straightforward random number generator library for Rust, inspired by Python's `random` module.

## Usage

```rs
 // Generate a random float in the range [0.0, 1.0)
random();

// Generate a random integer in the range [a, b]
randint(1, 10);

// Generate a random unsigned integer in the range [a, b]
randuint(1, 10);

// Generate a random float in the range [a, b]
uniform(1.0, 10.0);

// Choose a random element from a sequence
choice(&[1, 2, 3, 4, 5]);

// Shuffle a sequence in-place
shuffle(&mut [1, 2, 3, 4, 5]);
```