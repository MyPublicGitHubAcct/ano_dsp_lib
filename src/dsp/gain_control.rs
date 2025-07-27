#![allow(clippy::float_cmp)]

pub struct GainControl {
    current_gain: f64,
    target_gain: f64,
    step: f64,
    samples_left: usize,
}

impl GainControl {
    pub fn new(initial_gain: f64) -> Self {
        Self {
            current_gain: initial_gain,
            target_gain: initial_gain,
            step: 0.0,
            samples_left: 0,
        }
    }

    pub fn set_gain(&mut self, new_gain: f64, ramp_length: usize) {
        assert!(ramp_length >= 1, "ramp_length must be >= 1");
        self.target_gain = new_gain;
        self.samples_left = ramp_length;

        if ramp_length == 1 {
            self.current_gain = new_gain;
            self.step = 0.0;
            self.samples_left = 0;
        } else {
            self.step = (self.target_gain - self.current_gain) / (ramp_length as f64);
        }
    }

    pub fn process_sample(&mut self, x: f64) -> f64 {
        if self.samples_left > 0 {
            self.current_gain += self.step;
            self.samples_left -= 1;
            if self.samples_left == 0 {
                self.current_gain = self.target_gain;
                self.step = 0.0;
            }
        }
        x * self.current_gain
    }

    pub fn process_block(&mut self, input: &[f64], output: &mut [f64]) {
        assert_eq!(input.len(), output.len());
        for (i, &sample) in input.iter().enumerate() {
            output[i] = self.process_sample(sample);
        }
    }

    pub fn current_gain(&self) -> f64 {
        self.current_gain
    }

    pub fn samples_left(&self) -> usize {
        self.samples_left
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use std::vec;

    #[test]
    fn test_initial_gain() {
        let mut gc = GainControl::new(1.0);
        let y = gc.process_sample(0.5);
        assert!((y - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_instant_gain_change() {
        let mut gc = GainControl::new(1.0);
        gc.set_gain(2.0, 1);
        let y = gc.process_sample(1.0);
        assert!((y - 2.0).abs() < 1e-12);
        assert!((gc.current_gain() - 2.0).abs() < 1e-12);
        assert_eq!(gc.samples_left(), 0);
    }

    #[test]
    fn test_ramped_gain_single_samples() {
        let mut gc = GainControl::new(1.0);
        gc.set_gain(2.0, 4);

        let inputs = [1.0, 1.0, 1.0, 1.0];
        let expected_gains = [1.25, 1.5, 1.75, 2.0];

        for (expected_gain, &x) in expected_gains.iter().zip(&inputs) {
            let y = gc.process_sample(x);
            assert!((y - expected_gain).abs() < 1e-12);
        }

        assert!((gc.current_gain() - 2.0).abs() < 1e-12);
        assert_eq!(gc.samples_left(), 0);
    }

    #[test]
    fn test_process_block_constant_gain() {
        let mut gc = GainControl::new(1.0);
        gc.set_gain(1.5, 1);
        let input = vec![1.0; 5];
        let mut output = vec![0.0; 5];
        gc.process_block(&input, &mut output);
        for &y in &output {
            assert!((y - 1.5).abs() < 1e-12);
        }
    }

    #[test]
    fn test_process_block_with_ramp() {
        let mut gc = GainControl::new(1.0);
        gc.set_gain(2.0, 4);
        let input = vec![1.0; 4];
        let mut output = vec![0.0; 4];
        gc.process_block(&input, &mut output);
        let expected = [1.25, 1.5, 1.75, 2.0];
        for (o, &e) in output.iter().zip(&expected) {
            assert!((o - e).abs() < 1e-12);
        }
        assert!((gc.current_gain() - 2.0).abs() < 1e-12);
        assert_eq!(gc.samples_left(), 0);
    }

    #[test]
    fn test_process_block_partial_ramp() {
        let mut gc = GainControl::new(1.0);
        gc.set_gain(3.0, 6);

        let input1 = vec![1.0; 4];
        let mut output1 = vec![0.0; 4];
        gc.process_block(&input1, &mut output1);

        let expected1 = [
            1.3333333333333333,
            1.6666666666666665,
            2.0,
            2.333333333333333,
        ];
        for (o, &e) in output1.iter().zip(&expected1) {
            assert!((o - e).abs() < 1e-12);
        }

        let input2 = vec![1.0; 2];
        let mut output2 = vec![0.0; 2];
        gc.process_block(&input2, &mut output2);

        let expected2 = [2.6666666666666665, 3.0];
        for (o, &e) in output2.iter().zip(&expected2) {
            assert!((o - e).abs() < 1e-12);
        }

        assert!((gc.current_gain() - 3.0).abs() < 1e-12);
        assert_eq!(gc.samples_left(), 0);
    }
}
