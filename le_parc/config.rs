use nannou::color::{rgb8, IntoLinSrgba, Rgb};

#[derive(Copy, Clone)]
pub struct ColorRgb(u8, u8, u8);

pub const PALETTE: [&'static ColorRgb; 14] = [
    &ColorRgb(255, 227, 0),
    &ColorRgb(136, 193, 80),
    &ColorRgb(25, 168, 76),
    &ColorRgb(0, 144, 75),
    &ColorRgb(0, 115, 82),
    &ColorRgb(0, 82, 94),
    &ColorRgb(42, 43, 135),
    &ColorRgb(67, 29, 116),
    &ColorRgb(118, 8, 103),
    &ColorRgb(161, 0, 112),
    &ColorRgb(221, 0, 80),
    &ColorRgb(236, 74, 1),
    &ColorRgb(242, 127, 0),
    &ColorRgb(248, 165, 0),
];

impl Into<Rgb<u8>> for ColorRgb {
    fn into(self) -> Rgb<u8> {
        rgb8(self.0, self.1, self.2)
    }
}

impl IntoLinSrgba<f32> for &ColorRgb {
    fn into_lin_srgba(self) -> nannou::color::LinSrgba<f32> {
        rgb8(self.0, self.1, self.2).into_lin_srgba()
    }
}
