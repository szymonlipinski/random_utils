// Copyright 2016 by Szymon Lipiński.
// 
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A couple of random number generation utilities.
//!
//! # Usage
//!
//! ```toml
//! [dependencies]
//! random_utils = "0.1"
//! ```
//!

extern crate rand;
extern crate num;

use rand::{Rng};
use std::ops::Add;
use rand::distributions::range::SampleRange;
use std::fmt::Display;

use num::traits::{One};

/// Returns random number from the range [low, high].
///
/// The numbers are uniformly distributed over [low, high].
#[allow(dead_code)]
pub fn random_range<T>(low: T, high: T) -> T
    where T: PartialOrd + Add<Output=T> + SampleRange + Display + One
{
    
    if low > high {
        panic!("Low({}) is higher than high({}).", low, high);
    }

    let mut rng = rand::thread_rng();
    let higher = high + T::one();
    rng.gen_range::<T>(low, higher)
}


#[cfg(test)]
mod tests {

    use super::random_range;
    use std::collections::HashMap;
    use std::cmp::{max, min};

    #[test]
    #[should_panic(expected = "Low(10) is higher than high(5).")]
    fn random_range_bad_arguments() {
        random_range(10, 5);
    }

    #[test]
    fn random_range_equal_arguments() {
        for _ in 1..1000 {
            assert_eq!(random_range(10, 10), 10);
        }
    }

    #[test]
    /// A very simple test for randomness.
    ///
    /// Runs the rng function huge number of times.
    /// Checks the average (which should be (high+low)/2).
    ///
    /// Checks the number each generated number appeared,
    /// which should be 5% lower/higher than the average.
    ///
    /// Checks if each of the number has been generated.
    ///
    fn random_range_arguments() {
        let low = 1;
        let high = 1_000;
        let each_number_times = 10_000;
        let rand_numbers_count = (high-low+1) * each_number_times;
        let mut results: HashMap<i32, i32> = HashMap::new();

        for _ in 1..rand_numbers_count {
            let r = random_range(low, high);
            let counter = results.entry(r).or_insert(1);
            *counter += 1;
        }

        // calculate the average
        let mut sum: i64 = 0;
        let mut max_count = 0;
        let mut min_count = rand_numbers_count * 10;
        for (k ,v) in &results {
            sum = sum + (*v as i64) * (*k as i64);
            min_count = min(min_count, *v);
            max_count = max(max_count, *v);
        }
        assert_eq!(results.len(), (high-low+1) as usize);
        let average = sum as f32 / rand_numbers_count as f32;
        let expected_average = (high + low) as f32 / 2.0;

        assert!( expected_average <= average * 1.001);
        assert!( expected_average >= average * 0.999);
        
        assert!( (min_count as f32) > each_number_times as f32 * 0.95);
        assert!( (min_count as f32) < each_number_times as f32 * 1.05);
    }

}
