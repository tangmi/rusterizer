trait Interpolate {
	fn clamp(self, low: Self, high: Self) -> Self;
	
	/// val is from 0-1
	fn interpolate(min: Self, max: Self, val: Self) -> Self;
}

impl Interpolate for f32 {
	fn clamp(self, low: Self, high: Self) -> Self {
		Self::max(low, Self::min(self, high))
	}
	
	fn interpolate(min: Self, max: Self, val: Self) -> Self {
		min + (max - min) * val.clamp(0.0, 1.0)
	}
}

impl Interpolate for f64 {
	fn clamp(self, low: Self, high: Self) -> Self {
		Self::max(low, Self::min(self, high))
	}
	
	fn interpolate(min: Self, max: Self, val: Self) -> Self {
		min + (max - min) * val.clamp(0.0, 1.0)
	}
}
