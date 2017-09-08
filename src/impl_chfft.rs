
use super::*;
use chfft::*;

impl DftAlgorithm for CFft1D<f64> {
    fn convert(&mut self, source: &[c64]) -> Vec<c64> {
        self.forward(source)
    }

    fn convert_back(&mut self, source: &[c64]) -> Vec<c64> {
        self.backward(source)
    }
}
