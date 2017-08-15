use num::Complex;

pub fn x2(x: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    x * x + c
}

pub fn x3(x: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    x * x * x + c
}

pub fn q1(x: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    ((1.0 - x * x * x / 6.0) / (x - x * x / 2.0).powf(2.0)) + c
}

pub fn m1(x: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    (x*x + x) / x.log(::std::f64::consts::E) + c
}

