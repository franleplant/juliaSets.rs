use num::Complex;

pub fn x2(x: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    x * x + c
}

pub fn x3(x: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    x * x * x + c
}
