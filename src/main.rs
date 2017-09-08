
extern crate num_complex;
extern crate num_traits;

extern crate chfft;
extern crate fftw;
extern crate rustfft;

mod impl_chfft;
mod impl_fftw;
mod impl_rustfft;

use num_traits::*;
use num_complex::*;
use std::time::*;

type c64 = Complex64;

pub trait DftAlgorithm {
    fn convert(&mut self, source: &[c64]) -> Vec<c64>;
    fn convert_back(&mut self, source: &[c64]) -> Vec<c64>;
}

fn test_dft<A: DftAlgorithm>(arr: &[c64], mut algorithm: A, rep: usize) -> f64 {
    let mut tmp_arr = arr.to_vec();
    let start = Instant::now();
    for _ in 0..rep {
        let arr2 = algorithm.convert(&tmp_arr);
        tmp_arr = algorithm.convert_back(&arr2);
    }
    let end = start.elapsed();
    // assert_complexes_eq(&tmp_arr, arr);
    let total = (end.as_secs() * 1000000000_u64) + (end.subsec_nanos() as u64);
    return total as f64 / rep as f64 / 2.0;
}

fn main() {
    // TODO
}
