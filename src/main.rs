extern crate image;
extern crate num;
extern crate conrod;
extern crate elmesque;
extern crate gif;

use std::fs::File;
use std::path::Path;
use num::{Complex};
use conrod::color::{Color};
use std::f64::consts::PI;

static MAX_ITER: i64 = 1000;
static N: i64 = 100;
static M: i64 = 100;
static RANGE_X: f64 = 3.;
static RANGE_Y: f64 = 3.;
const STEP: f32 = 5.0;
const PHASE: f32 = 0.0;


fn get_z0(center: &Complex<f64>, range_x: f64, range_y: f64 ) -> Complex<f64> {
    Complex::new(
        center.re - range_x / 2.0,
        center.im + range_y / 2.0
    )
}

fn get_delta(range_x: f64, range_y: f64, width: i64, height: i64) -> (f64, f64) {
    (
        range_x / (width as f64),
        range_y / (height as f64)
    )
}

fn color_by_index(k: i64) -> image::Rgba<u8> {

    let hue = ((STEP * (k as f32) + PHASE)% 360.0) * 2.0 * PI as f32 / 360.0;
    //println!("k {}, hue {}", k, hue);
    let color = Color::Hsla(hue as f32, 0.7f32, 0.6f32, 0.9f32);
    let elmesque::color::Rgba(r, g, b, a) = color.to_rgb();
    let r = r * 255f32;
    let g = g * 255f32;
    let b = b * 255f32;
    let a = a * 255f32;
    //println!("{}, {}, {}, {}", r,g,b,a);
    image::Rgba([r as u8, g as u8, b as u8, a as u8])
}

fn fractal<F>(f: F, z0: &Complex<f64>, delta: &(f64, f64), width: i64, height: i64, max_iter: i64) -> image::RgbaImage
    where F: Fn(Complex<f64>) -> Complex<f64> {

    static BAILOUT: f64 = 2.0;

    image::ImageBuffer::from_fn(width as u32, height as u32, |i, j| {
        let mut z = Complex::new(
            z0.re + delta.0 * (i as f64),
            z0.im - delta.1 * (j as f64)
        );

        let mut k = 0;
        while (z.norm_sqr() < BAILOUT) && (k < max_iter) {
            z = f(z);

            k += 1;
        }

        color_by_index(k)
    })
}

fn main() {
    let center = Complex::new(0.0, 0.0);
    let z0 = get_z0(&center, RANGE_X, RANGE_Y);
    let delta = get_delta(RANGE_X, RANGE_Y, N, M);
    let f = |z| z*z + Complex::new( 0.279, 0.0 );

    let mut img = fractal(f, &z0, &delta, N, M, MAX_ITER);

    // The following commented lines are for saving a single "frame" into a png
    // Save the image as “fractal.png”
    //let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();
    // We must indicate the image’s color type and what format to save as
    //let _ = image::ImageRgba8(img).save(fout, image::PNG);

    let ref mut fout = File::create(&Path::new("fractal.gif")).unwrap();
    // Create frame from data
    let frame = gif::Frame::from_rgba(N as u16, M as u16, &mut *img);
    // Create encoder
    //let mut image = Vec::new();
    let encoder = gif::Encoder::new(&mut *fout, N as u16, M as u16);
    // Write header to file
    let mut encoder = encoder.write_global_palette(&[]).unwrap();
    // Write frame to file
    encoder.write_frame(&frame).unwrap();
}


#[test]
fn test_get_z0() {
    let center = Complex::new(0.0, 0.0);
    let z0 = get_z0(&center, 2.0, 2.0);
    assert!(z0.re == -1.0);
    assert!(z0.im == 1.0);
}

#[test]
fn test_get_delta() {
    let (dx, dy) = get_delta(2.0, 2.0, 2, 2);
    assert!(dx == 1.0);
    assert!(dy == 1.0);
}
