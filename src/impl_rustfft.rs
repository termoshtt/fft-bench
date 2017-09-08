
use super::*;
use rustfft::*;
use std::sync::Arc;

pub struct RustFft<T> {
    len: usize,
    forward: Arc<FFT<T>>,
    backward: Arc<FFT<T>>,
    scaler: f64,
}

impl<T: FFTnum> RustFft<T> {
    pub fn new(len: usize) -> Self {
        Self {
            len: len,
            forward: FFTplanner::new(false).plan_fft(len),
            backward: FFTplanner::new(true).plan_fft(len),
            scaler: 1.0 / len as f64,
        }
    }
}

impl DftAlgorithm for RustFft<f64> {
    fn convert(&mut self, source: &[c64]) -> Vec<c64> {
        let mut dsource = source.to_vec();
        let mut target = vec![zero(); source.len()];
        self.forward.process(&mut dsource, &mut target);
        target
    }

    fn convert_back(&mut self, source: &[c64]) -> Vec<c64> {
        let mut dsource = source.to_vec();
        let mut target = vec![zero(); source.len()];
        self.backward.process(&mut dsource, &mut target);
        for t in target.iter_mut() {
            *t = t.scale(self.scaler);
        }
        target
    }
}
