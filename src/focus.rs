pub struct Focus(usize);

impl Focus {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn next(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    pub fn prev(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    pub fn focused(&self, index: usize) -> bool {
        self.0 == index
    }

    pub(crate) fn clamp(&mut self, min: usize, max: usize) {
        self.0 = self.0.clamp(min, max);
    }
}
