pub mod bit_crush;
pub mod envelope;
pub mod noise;
pub mod passband;
pub mod synth;
pub mod traits;
pub mod waveform;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(test)]
mod tests;

pub struct Samples<T> {
    pub sample_rate: u32,
    pub samples: Vec<T>,
}

#[inline]
pub(crate) fn lerp(prev: f64, curr: f64, p: f64) -> f64 {
    (1. - p) * prev + p * curr
}

#[doc(hidden)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn name() -> wasm_bindgen::JsValue {
    wasm_bindgen::JsValue::from_str("rs-fxr")
}
