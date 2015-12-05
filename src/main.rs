use std::ops::Add;

static MAX_ITER: i64 = 10;
static BAILOUT: f64 = 2.0;
static N: i64 = 10;
static M: i64 = 10;
// f64
//
//


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

fn getZ0(center: &Complex, rangeX: f64, rangeY: f64 ) -> Complex {
    Complex::new(
        center.x - rangeX / 2.0,
        center.y + rangeY / 2.0
    )
}

fn getDelta(rangeX: f64, rangeY: f64, width: i64, height: i64) -> (f64, f64) {
    (
        rangeX / (width as f64),
        rangeY / (height as f64)
    )
}

fn fractal(z0: &Complex, delta: &(f64, f64), width: i64, height: i64) {
    let C: Complex  = Complex{ x: 0.279, y:0.0 };
    let mut x: f64;
    let mut y: f64;
    let mut u: f64;
    let mut v: f64;
    let mut xx: f64;
    let mut yy: f64;


    for j in 0..height {
        for i in 0..width {
            let mut z = Complex::new(
                z0.x + delta.0 * (i as f64),
                z0.y - delta.1 * (j as f64)
            );

            let mut k = 0;
            while (z.sqnorm() < BAILOUT) && (k < MAX_ITER) {
                z = z.pow2() + C;

                k += 1;
            }
            println!("k {}", k);


        }
    }
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
    let rangeX = 2.;
    let rangeY = 2.;
    let z0 = getZ0(&center, rangeX, rangeY);
    let delta = getDelta(rangeX, rangeY, N, M);
    fractal(&z0, &delta, N, M);
}


#[test]
fn test_getZ0() {
    let center = Complex::new(0.0, 0.0);
    let z0 = getZ0(&center, 2.0, 2.0);
    assert!(z0.x == -1.0);
    assert!(z0.y == 1.0);
}

#[test]
fn test_getDelta() {
    let (dx, dy) = getDelta(2.0, 2.0, 2, 2);
    assert!(dx == 1.0);
    assert!(dy == 1.0);
}
