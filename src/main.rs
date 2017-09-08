
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

#[allow(non_camel_case_types)]
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
    let total = (end.as_secs() * 1000_000_000) + (end.subsec_nanos() as u64);
    return total as f64 / rep as f64 / 2.0;
}

fn main() {
    println!("n,chfft,fftw,rustfft");
    for a in 1..14 {
        let n: usize = 2.pow(a);
        let a = vec![c64::new(1.0, 0.0); n];
        let chfft = chfft::CFft1D::with_len(n);
        let fftw = impl_fftw::Fftw64::new(n);
        let rustfft = impl_rustfft::RustFft::new(n);
        println!(
            "{},{},{},{}",
            n,
            test_dft(&a, chfft, 10),
            test_dft(&a, fftw, 10),
            test_dft(&a, rustfft, 10),
        );
    }
}
