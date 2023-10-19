/*
    Simple and stright-forward random generator (wrapper to rand crate) - Inpired by the Python's random module
*/

use rand::prelude::*;

/// Generates a random float in the range `[0.0, 1.0)`
pub fn random() -> f64 {
    rand::thread_rng().gen()
}

/// Generates a random integer in the range `[a, b]`
pub fn randint(a: i64, b: i64) -> i64 {
    random() as i64 * (b - a) + a
}

/// Generates a random unsigned integer in the range `[a, b]`
pub fn randuint(a: usize, b: usize) -> usize {
    random() as usize * (b - a) + a
}

/// Generates a random float in the range `[a, b]`
pub fn uniform(a: f64, b: f64) -> f64 {
    random() * (b - a) + a
}

/// Picks a random element from a sequence and returns a reference to it
pub fn choice<T>(seq: &[T]) -> &T {
    &seq[randuint(0, seq.len())]
}

/// Shuffles a sequence in-place
pub fn shuffle<T>(seq: &mut [T]) {
    for i in 0..seq.len() {
        let j = randuint(0, seq.len());
        seq.swap(i, j);
    }
}

