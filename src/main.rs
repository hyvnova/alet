use alet::*;


// Macro to print out a expression and its result. Example: `debug!(1+1)` will print out `1+1 = 2`
macro_rules! debug{
    ($e:expr) => {
        println!("{} = {:?}", stringify!{$e}, $e)
    };
}

fn main() {
    debug!(random());
    debug!(randint(1, 10));
    debug!(randuint(1, 10));
    debug!(uniform(1.0, 10.0));
    debug!(choice(&[1, 2, 3, 4, 5]));
    debug!(shuffle(&mut [1, 2, 3, 4, 5]));
}   