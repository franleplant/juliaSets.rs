extern crate image;
use std::fs::File;
use std::path::Path;
use std::ops::Add;
use image::{ImageBuffer, Rgba};

static MAX_ITER: i64 = 1000;
static BAILOUT: f64 = 2.0;
static N: i64 = 100;
static M: i64 = 100;


#[derive(Debug, Copy, Clone)]
struct Complex {
    x: f64,
    y: f64
}

impl Complex {
    fn new(x: f64, y: f64) -> Complex {
        Complex { x:x, y:y}
    }

    fn sqnorm(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    fn pow2(&self) -> Complex {
        Complex::new(
            self.x.powi(2) - self.y.powi(2),
            2.0 * self.x * self.y
        )
    }

}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex { x: self.x + other.x, y: self.y + other.y }
    }
}

fn get_z0(center: &Complex, range_x: f64, range_y: f64 ) -> Complex {
    Complex::new(
        center.x - range_x / 2.0,
        center.y + range_y / 2.0
    )
}

fn get_delta(range_x: f64, range_y: f64, width: i64, height: i64) -> (f64, f64) {
    (
        range_x / (width as f64),
        range_y / (height as f64)
    )
}

fn fractal(z0: &Complex, delta: &(f64, f64), width: i64, height: i64) {
    let c: Complex  = Complex{ x: 0.279, y:0.0 };


    // Create a new ImgBuf with width: imgx and height: imgy
    //let mut imgbuf = ImageBuffer::new(width as u32, height as u32);

    let img = ImageBuffer::from_fn(width as u32, height as u32, |i, j| {
        let mut z = Complex::new(
            z0.x + delta.0 * (i as f64),
            z0.y - delta.1 * (j as f64)
        );

        let mut k = 0;
        while (z.sqnorm() < BAILOUT) && (k < MAX_ITER) {
            z = z.pow2() + c;

            k += 1;
        }
        println!("k {}", k);

        Rgba([((k*10) % 255) as u8,255u8,255u8,255u8])
    });
    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgba8(img).save(fout, image::PNG);
    //let color = Rgba([1,1,1,1]);

    //for j in 0..height {
        //for i in 0..width {
            //let mut z = Complex::new(
                //z0.x + delta.0 * (i as f64),
                //z0.y - delta.1 * (j as f64)
            //);

            //let mut k = 0;
            //while (z.sqnorm() < BAILOUT) && (k < MAX_ITER) {
                //z = z.pow2() + C;

                //k += 1;
            //}
            //println!("k {}", k);


        //}
    //}
      //pos = 4 * M * j
      //col = 4 * i

      //let {red, green, blue, alpha} = initialColor.shiftHue(k * COLOR_K % 360).toRGB()
      //B[pos + col + 0] = red*255
      //B[pos + col + 1] = green*255
      //B[pos + col + 2] = blue*255
      //B[pos + col + 3] = alpha*255

}

fn main() {
    let center = Complex::new(0.0, 0.0);
    let range_x = 2.;
    let range_y = 2.;
    let z0: Complex = get_z0(&center, range_x, range_y);
    let delta = get_delta(range_x, range_y, N, M);
    fractal(&z0, &delta, N, M);
}


#[test]
fn test_get_z0() {
    let center = Complex::new(0.0, 0.0);
    let z0 = get_z0(&center, 2.0, 2.0);
    assert!(z0.x == -1.0);
    assert!(z0.y == 1.0);
}

#[test]
fn test_get_delta() {
    let (dx, dy) = get_delta(2.0, 2.0, 2, 2);
    assert!(dx == 1.0);
    assert!(dy == 1.0);
}
