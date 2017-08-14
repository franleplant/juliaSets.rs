use image;
use std::f64::consts::PI;
use std::default::Default;
use conrod::color::{Rgba, Color};

use super::{Colorizer};

pub struct SimpleColorizer {
    step: f32,
    phase: f32,
    saturation: f32,
    lightness: f32,
    alpha: f32,
}

impl Default for SimpleColorizer {
    fn default() -> SimpleColorizer {
        SimpleColorizer {
            step: 5.0,
            phase: 0.0,
            saturation: 0.7,
            lightness: 0.6,
            alpha: 0.9,
        }
    }
}

impl Colorizer for SimpleColorizer {
    fn calc_color(&self, k: u32) -> image::Rgba<u8> {
        // TODO what?
        let conv_factor: f32 = 2.0f32 * PI as f32 / 360.0f32;
        let hue = ((self.step * k as f32 + self.phase) % 360.0f32) * conv_factor;
        let Rgba(r, g, b, a) = Color::Hsla(hue, self.saturation, self.lightness, self.alpha).to_rgb();
        let r = (r * 255f32) as u8;
        let g = (g * 255f32) as u8;
        let b = (b * 255f32) as u8;
        let a = (a * 255f32) as u8;

        image::Rgba([r, g, b, a])
    }
}
