pub struct Interval {
    duration: f32,
    start: f32,
    end: f32,
}

impl Interval {
    pub fn value(&self, t: f32) -> f32 {
        if t >= 0. && t <= self.duration {
            crate::lerp(self.start, self.end, t / self.duration)
        } else {
            0.
        }
    }

    pub fn new(duration: f32, start: f32, end: f32) -> Option<Self> {
        if duration.is_nan()
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
    vibrato: Option<(f32, f32)>,
}

impl Envelope {
    pub fn from_points(points: Vec<(f32, f32)>, vibrato: Option<(f32, f32)>) -> Option<Self> {
        if let Some((depth, freq)) = vibrato {
            if depth < 0. || depth > 1. || freq < 0. {
                return None;
            }
        }
        if points.is_empty() {
            return Some(Self {
                segments: vec![],
                vibrato,
            });
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
        Some(Self { segments, vibrato })
    }

    pub fn from_duration(
        amp: f32,
        attack: f32,
        sustain: f32,
        decay: f32,
        sustain_punch: f32,
        vibrato: Option<(f32, f32)>,
    ) -> Option<Self> {
        if let Some((depth, freq)) = vibrato {
            if depth < 0. || depth > 1. || freq < 0. {
                return None;
            }
        }
        if amp <= 0. || attack < 0. || sustain < 0. || decay < 0. || attack + sustain + decay <= 0.
        {
            return None;
        }
        let mut segments = vec![];
        if let Some(seg) = Interval::new(attack, 0., amp) {
            segments.push(seg);
        }
        if let Some(seg) = Interval::new(sustain, amp * (1. + sustain_punch), amp) {
            segments.push(seg);
        }
        if let Some(seg) = Interval::new(decay, amp, 0.) {
            segments.push(seg);
        }
        Some(Self { segments, vibrato })
    }

    pub fn new_simple(duration: f32, value: f32) -> Option<Self> {
        let interval = Interval::new(duration, value, value)?;
        Some(Envelope {
            segments: vec![interval],
            vibrato: None,
        })
    }
}

impl crate::traits::Proc for Envelope {
    fn value(&self, t: f32) -> f32 {
        let mut _t = t;
        for s in self.segments.iter() {
            if _t < s.duration {
                if let Some((depth, freq)) = self.vibrato {
                    return s.value(_t) * (1. - depth * (std::f32::consts::TAU * freq * t).cos());
                } else {
                    return s.value(_t);
                }
            }
            _t -= s.duration;
        }
        0.
    }
}

impl crate::traits::Duration for Envelope {
    fn duration(&self) -> f32 {
        if self.segments.is_empty() {
            return 0.;
        }
        self.segments.iter().map(|s| s.duration).sum()
    }
}
