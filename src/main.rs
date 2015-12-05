extern crate image;
extern crate num;
extern crate conrod;

use std::fs::File;
use std::path::Path;
use image::{ImageBuffer, Rgba};
use num::{Complex};
use conrod::color::{Color};

static MAX_ITER: i64 = 1000;
static BAILOUT: f64 = 2.0;
static N: i64 = 1000;
static M: i64 = 1000;
static RANGE_X: f64 = 3.;
static RANGE_Y: f64 = 3.;


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

fn fractal(z0: &Complex<f64>, delta: &(f64, f64), width: i64, height: i64) {
    let c  = Complex::new( 0.279, 0.0 );

    let img = ImageBuffer::from_fn(width as u32, height as u32, |i, j| {
        let mut z = Complex::new(
            z0.re + delta.0 * (i as f64),
            z0.im - delta.1 * (j as f64)
        );

        let mut k = 0;
        while (z.norm_sqr() < BAILOUT) && (k < MAX_ITER) {
            //println!("k {}, z {}, norm {}",k, z, z.norm_sqr());
            z = z * z + c;

            k += 1;
        }

        // TODO: abstract this in a function so I can test it properly
        let hue = ((5.0 * (k as f32) + 0.0)% 360.0) * 2.0 * 3.14 / 360.0;
        //let hue = k as f64 % 360;
        //println!("k {}, hue {}", k, hue);
        let color = Color::Hsla(hue as f32, 0.7f32, 0.6f32, 0.9f32);
        let rgba = color.to_rgb();
        let r = rgba.0 * 255f32;
        let g = rgba.1 * 255f32;
        let b = rgba.2 * 255f32;
        let a = rgba.3 * 255f32;
        //println!("{}, {}, {}, {}", r,g,b,a);
        Rgba([r as u8, g as u8, b as u8, a as u8])
    });
    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgba8(img).save(fout, image::PNG);
}

fn main() {
    let center = Complex::new(0.0, 0.0);
    let z0 = get_z0(&center, RANGE_X, RANGE_Y);
    let delta = get_delta(RANGE_X, RANGE_Y, N, M);
    fractal(&z0, &delta, N, M);
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
