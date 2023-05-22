use realfft::{num_complex::Complex, num_traits::Zero, FftNum, RealFftPlanner};

pub trait Filterable {
    fn low_pass(self, cutoff_freq: f64) -> Self;
    fn high_pass(self, cutoff_freq: f64) -> Self;
}

fn filter<T>(
    planner: &mut RealFftPlanner<T>,
    samples: &mut Vec<T>,
    sample_rate: f64,
    cutoff_freq: f64,
    high_pass: bool,
) where
    T: FftNum,
{
    let len = samples.len();
    let fft = planner.plan_fft_forward(len);
    let spectrum = &mut fft.make_output_vec();
    fft.process(samples, spectrum).unwrap();

    let cutoff = ((cutoff_freq / sample_rate) * spectrum.len() as f64) as usize;
    if high_pass {
        spectrum
            .iter_mut()
            .take(cutoff)
            .for_each(|f| *f = Complex::zero());
    } else {
        spectrum
            .iter_mut()
            .skip(cutoff)
            .for_each(|f| *f = Complex::zero());
    }

    let fft = planner.plan_fft_inverse(len);
    fft.process(spectrum, samples).unwrap();
}

impl Filterable for crate::Samples<f64> {
    fn low_pass(mut self, cutoff_freq: f64) -> Self {
        let mut planner = RealFftPlanner::<f64>::new();
        filter(
            &mut planner,
            &mut self.samples,
            self.sample_rate as f64,
            cutoff_freq,
            false,
        );
        let len = self.samples.len() as f64;
        self.samples.iter_mut().for_each(|s| *s /= len);
        self
    }

    fn high_pass(mut self, cutoff_freq: f64) -> Self {
        let mut planner = RealFftPlanner::<f64>::new();
        filter(
            &mut planner,
            &mut self.samples,
            self.sample_rate as f64,
            cutoff_freq,
            true,
        );
        let len = self.samples.len() as f64;
        self.samples.iter_mut().for_each(|s| *s /= len);
        self
    }
}

impl Filterable for crate::Samples<f32> {
    fn low_pass(mut self, cutoff_freq: f64) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        filter(
            &mut planner,
            &mut self.samples,
            self.sample_rate as f64,
            cutoff_freq,
            false,
        );
        let len = self.samples.len() as f32;
        self.samples.iter_mut().for_each(|s| *s /= len);
        self
    }

    fn high_pass(mut self, cutoff_freq: f64) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        filter(
            &mut planner,
            &mut self.samples,
            self.sample_rate as f64,
            cutoff_freq,
            true,
        );
        let len = self.samples.len() as f32;
        self.samples.iter_mut().for_each(|s| *s /= len);
        self
    }
}
