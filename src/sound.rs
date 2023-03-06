pub trait Sound: Send {
    fn next(&mut self) -> Option<f64>;

    fn reset(&mut self);

    fn duration(&self) -> f64;

    fn render(&mut self, sample_rate: f64) -> Option<Vec<f64>> {
        if !self.duration().is_normal() || self.duration() <= 0. {
            return None;
        }
        let len = (self.duration() * sample_rate) as usize;
        let mut render = Vec::with_capacity(len);
        for _ in 0..len {
            if let Some(s) = self.next() {
                render.push(s)
            } else {
                return None;
                // render.push(0.)
            }
        }
        Some(render)
    }
}
