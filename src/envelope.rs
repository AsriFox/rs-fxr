pub struct Interval {
    duration: f64,
    start: f64,
    end: f64,
}

impl Interval {
    pub fn value(&self, t: f64) -> f64 {
        if t < 0. || t > self.duration {
            return 0.;
        }
        let t = t / self.duration;
        (1. - t) * self.start + t * self.end
    }

    pub fn new(duration: f64, start: f64, end: f64) -> Option<Self> {
        if !duration.is_normal()
            || duration <= 0.
            || !(start.is_normal() || start == 0.)
            || !(end.is_normal() || end == 0.)
        {
            None
        } else {
            Some(Self {
                duration,
                start,
                end,
            })
        }
    }
}

pub struct Envelope {
    segments: Vec<Interval>,
}

impl Envelope {
    pub fn new(segments: Vec<Interval>) -> Self {
        Self { segments }
    }

    pub fn from_points(points: Vec<(f64, f64)>) -> Option<Self> {
        if points.is_empty() {
            return Some(Self { segments: vec![] });
        }
        let mut segments = Vec::with_capacity(points.len() - 1);
        let mut points = points.into_iter();
        let (mut t_prev, mut v_prev) = points.next()?;
        if t_prev > 0. {
            segments.push(Interval::new(t_prev, 0., v_prev)?);
        }
        while let Some((t, v)) = points.next() {
            if let Some(s) = Interval::new(t - t_prev, v_prev, v) {
                segments.push(s);
                t_prev = t;
                v_prev = v;
            }
        }
        Some(Self { segments })
    }
}

impl crate::sound::Proc for Envelope {
    fn value(&self, t: f64) -> f64 {
        let mut t = t;
        for s in self.segments.iter() {
            if t < s.duration {
                return s.value(t);
            }
            t -= s.duration;
        }
        0.
    }
}

impl crate::sound::Duration for Envelope {
    fn duration(&self) -> f64 {
        if self.segments.is_empty() {
            return 0.;
        }
        self.segments.iter().map(|s| s.duration).sum()
    }
}
