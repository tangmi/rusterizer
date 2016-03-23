trait Clamp {
	fn clamp(self, low: Self, high: Self) -> Self;
}

impl Clamp for f64 {
    fn clamp(self, low: Self, high: Self) -> Self {
        Self::max(low, Self::min(self, high))
    }
}

pub trait Interpolate {
    /// val is from 0-1
    fn interpolate(min: Self, max: Self, val: Self) -> Self;
}

impl Interpolate for f64 {
    fn interpolate(min: Self, max: Self, val: Self) -> Self {
        min + (max - min) * val.clamp(0.0, 1.0)
    }
}
