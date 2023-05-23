use crate::{envelope::Envelope, traits::Proc};

pub struct Sine {
    freq: Envelope,
}
impl Sine {
    pub fn new(freq: Envelope) -> Self {
        Self { freq }
    }
    pub fn new_simple(freq: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. {
            None
        } else {
            let freq = Envelope::from_duration(freq, 0., f64::INFINITY, 0., 0., None).unwrap();
            Some(Self { freq })
        }
    }
}
impl Proc for Sine {
    fn value(&self, t: f64) -> f64 {
        (std::f64::consts::TAU * self.freq.value(t) * t).sin()
    }
}

pub struct Triangle {
    freq: Envelope,
}
impl Triangle {
    pub fn new(freq: Envelope) -> Self {
        Self { freq }
    }
    pub fn new_simple(freq: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. {
            None
        } else {
            let freq = Envelope::from_duration(freq, 0., f64::INFINITY, 0., 0., None).unwrap();
            Some(Self { freq })
        }
    }
}
impl Proc for Triangle {
    fn value(&self, t: f64) -> f64 {
        let v = (self.freq.value(t) * t).fract();
        if v < 0.25 {
            4. * v
        } else if v < 0.75 {
            2. - 4. * v
        } else {
            -4. + 4. * v
        }
    }
}

pub struct Sawtooth {
    freq: Envelope,
}
impl Sawtooth {
    pub fn new(freq: Envelope) -> Self {
        Self { freq }
    }
    pub fn new_simple(freq: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. {
            None
        } else {
            let freq = Envelope::from_duration(freq, 0., f64::INFINITY, 0., 0., None).unwrap();
            Some(Self { freq })
        }
    }
}
impl Proc for Sawtooth {
    fn value(&self, t: f64) -> f64 {
        (self.freq.value(t) * t).fract() * 2. - 1.
    }
}

pub struct Breaker {
    freq: Envelope,
}
impl Breaker {
    pub fn new(freq: Envelope) -> Self {
        Self { freq }
    }
    pub fn new_simple(freq: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. {
            None
        } else {
            let freq = Envelope::from_duration(freq, 0., f64::INFINITY, 0., 0., None).unwrap();
            Some(Self { freq })
        }
    }
}
impl Proc for Breaker {
    fn value(&self, t: f64) -> f64 {
        const BREAKER_OFFSET: f64 = 0.86602540378443864676372317075294; // f64::sqrt(0.75);
        let v = (self.freq.value(t) * t + BREAKER_OFFSET).fract();
        -1. + 2. * (1. - 2. * v * v).abs()
    }
}

pub struct Tangent {
    freq: Envelope,
    cutoff: f64,
}
impl Tangent {
    pub fn default(freq: Envelope) -> Self {
        Self { freq, cutoff: 0.15 }
    }
    pub fn new_simple(freq: f64, cutoff: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. || !cutoff.is_normal() || cutoff <= 0. {
            None
        } else {
            let freq = Envelope::from_duration(freq, 0., f64::INFINITY, 0., 0., None).unwrap();
            Some(Self { freq, cutoff })
        }
    }
    pub fn default_simple(freq: f64) -> Option<Self> {
        Self::new_simple(freq, 0.15)
    }
}
impl Proc for Tangent {
    fn value(&self, t: f64) -> f64 {
        ((std::f64::consts::PI * self.freq.value(t) * t).tan() / self.cutoff).clamp(-1., 1.)
    }
}

pub struct Square {
    freq: Envelope,
    square_duty: Envelope,
}
impl Square {
    pub fn new(freq: Envelope, square_duty: Envelope) -> Self {
        Self { freq, square_duty }
    }
    pub fn new_simple(freq: f64, square_duty: f64) -> Option<Self> {
        if !freq.is_normal()
            || freq <= 0.
            || !square_duty.is_normal()
            || square_duty <= 0.
            || square_duty >= 1.
        {
            None
        } else {
            let freq = Envelope::from_duration(freq, 0., f64::INFINITY, 0., 0., None).unwrap();
            let square_duty =
                Envelope::from_duration(square_duty, 0., f64::INFINITY, 0., 0., None).unwrap();
            Some(Self { freq, square_duty })
        }
    }
    pub fn default_simple(freq: f64) -> Option<Self> {
        Self::new_simple(freq, 0.5)
    }
}
impl Proc for Square {
    fn value(&self, t: f64) -> f64 {
        let v = (self.freq.value(t) * t).fract();
        if v < self.square_duty.value(t) {
            1.
        } else {
            -1.
        }
    }
}
