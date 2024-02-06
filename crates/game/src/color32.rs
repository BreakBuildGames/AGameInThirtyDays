#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct Linear32 {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[allow(dead_code)]
impl Linear32 {
    /// Almost black with a touch of green
    pub const DARK_JUNGLE_GREEN: Self = Self::from_rgb(0.102, 0.141, 0.129);
    /// Grape like purplee
    pub const PERSIAN_INDIGO: Self = Self::from_rgb(0.20, 0.0, 0.30);
    /// Dirty Whitee
    pub const GAINSBORO: Self = Self::from_rgb(0.79, 0.92, 0.87);
    /// It's really nice to look at
    pub const UNITY_YELLOW: Self = Self::from_rgb(1.0, 0.92, 0.016);

    /// The color Black
    pub const BLACK: Self = Self::from_rgb(0.0, 0.0, 0.0);
    /// The color Red
    pub const RED: Self = Self::from_rgb(1.0, 0.0, 0.0);
    /// The color Blue
    pub const BLUE: Self = Self::from_rgb(0.0, 0.0, 1.0);
    /// The color Green
    pub const GREEN: Self = Self::from_rgb(0.0, 1.0, 0.0);
    /// The color Yellow
    pub const YELLOW: Self = Self::from_rgb(1.0, 1.0, 0.0);
    /// The color White
    pub const WHITE: Self = Self::from_rgb(1.0, 1.0, 1.0);

    #[must_use]
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
            a: 1.0,
        }
    }

    #[must_use]
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    #[must_use]
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const f32 {
        (self as *const Self).cast()
    }

    #[must_use]
    pub const fn as_rgb(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    #[must_use]
    pub const fn as_rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
