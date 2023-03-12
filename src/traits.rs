pub trait Proc {
    fn value(&self, t: f64) -> f64;
}

pub trait ProcState {
    fn next_value(&mut self, t: f64) -> f64;
}

pub trait Duration {
    fn duration(&self) -> f64;
}
