
use super::*;
use fftw::{Pair, SIGN, FLAG};

pub struct Fftw64 {
    pair: Pair<c64, c64>,
    scaler: f64,
}

impl Fftw64 {
    pub fn new(len: usize) -> Self {
        Self {
            pair: Pair::c2c_1d(len, SIGN::FFTW_FORWARD, FLAG::FFTW_ESTIMATE),
            scaler: 1.0 / len as f64,
        }
    }
}

impl DftAlgorithm for Fftw64 {
    fn convert(&mut self, source: &[Complex<f64>]) -> Vec<Complex<f64>> {
        for (s, d) in source.iter().zip(self.pair.field.iter_mut()) {
            *d = *s;
        }
        self.pair.forward();
        self.pair.coef.as_slice().to_vec()
    }

    fn convert_back(&mut self, source: &[Complex<f64>]) -> Vec<Complex<f64>> {
        for (s, d) in source.iter().zip(self.pair.coef.iter_mut()) {
            *d = *s;
        }
        self.pair.backward();

        self.pair
            .field
            .iter()
            .map(|x| x.scale(self.scaler))
            .collect::<Vec<_>>()
    }
}
