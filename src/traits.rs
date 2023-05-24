pub trait Proc {
    fn value(&self, t: f32) -> f32;
}

pub trait Duration {
    fn duration(&self) -> f32;
}

pub trait Synth: Iterator<Item = f32> + Duration + Send {}
