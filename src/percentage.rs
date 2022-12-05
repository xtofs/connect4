use std::fmt::Display;

pub struct Percent(f32);

impl Percent {
    pub fn from_total(n: usize, total: usize) -> Self {
        Self(n as f32 / total as f32)
    }
}

impl Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1$}%", 100.0 * self.0, f.precision().unwrap_or(1))
    }
}
