use crate::traits::{Duration, Synth};

pub enum BitCrush {
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    B10,
    B11,
    B12,
    B13,
    B14,
    B15,
    B16,
}

impl BitCrush {
    pub fn new(bits: usize) -> Option<Self> {
        match bits {
            1 => Some(Self::B1),
            2 => Some(Self::B2),
            3 => Some(Self::B3),
            4 => Some(Self::B4),
            5 => Some(Self::B5),
            6 => Some(Self::B6),
            7 => Some(Self::B7),
            8 => Some(Self::B8),
            9 => Some(Self::B9),
            10 => Some(Self::B10),
            11 => Some(Self::B11),
            12 => Some(Self::B12),
            13 => Some(Self::B13),
            14 => Some(Self::B14),
            15 => Some(Self::B15),
            16 => Some(Self::B16),
            _ => None,
        }
    }

    pub fn mask(&self) -> i16 {
        match self {
            BitCrush::B1 => 0b0100000000000000,
            BitCrush::B2 => 0b0110000000000000,
            BitCrush::B3 => 0b0111000000000000,
            BitCrush::B4 => 0b0111100000000000,
            BitCrush::B5 => 0b0111110000000000,
            BitCrush::B6 => 0b0111111000000000,
            BitCrush::B7 => 0b0111111100000000,
            BitCrush::B8 => 0b0111111110000000,
            BitCrush::B9 => 0b0111111111000000,
            BitCrush::B10 => 0b0111111111100000,
            BitCrush::B11 => 0b0111111111110000,
            BitCrush::B12 => 0b0111111111111000,
            BitCrush::B13 => 0b0111111111111100,
            BitCrush::B14 => 0b0111111111111110,
            BitCrush::B15 => 0b0111111111111111,
            BitCrush::B16 => unimplemented!(),
        }
    }

    pub fn apply<S>(self, sound: S) -> BitCrushedSound<S>
    where
        S: Synth,
    {
        BitCrushedSound {
            sound,
            bit_mask: self,
        }
    }
}

pub struct BitCrushedSound<S>
where
    S: Synth,
{
    sound: S,
    bit_mask: BitCrush,
}

impl<S> Synth for BitCrushedSound<S> where S: Synth {}

unsafe impl<S> Send for BitCrushedSound<S> where S: Synth {}

impl<S> Duration for BitCrushedSound<S>
where
    S: Synth,
{
    fn duration(&self) -> f32 {
        self.sound.duration()
    }
}

impl<S> Iterator for BitCrushedSound<S>
where
    S: Synth,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample) = self.sound.next() {
            let sample = (f32::clamp(sample, -1., 1.) * i16::MAX as f32) as i16;
            if let BitCrush::B16 = self.bit_mask {
                Some(sample as f32 / i16::MAX as f32)
            } else {
                let sample = if sample > 0 {
                    sample & self.bit_mask.mask()
                } else if sample < 0 {
                    -(-sample & self.bit_mask.mask())
                } else {
                    return Some(0.);
                };
                Some(sample as f32 / i16::MAX as f32)
            }
        } else {
            None
        }
    }
}
