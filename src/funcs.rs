use num::Complex;

pub fn f0(x: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    x * x + c
}

pub fn f1(x: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    x * x * x + c
}

pub fn f10(x: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    x.exp() + c
}

pub fn f100(x: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    ((1.0 - x * x * x / 6.0) / (x - x * x / 2.0).powf(2.0)) + c
}

pub fn f200(x: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    (x*x + x) / x.log(::std::f64::consts::E) + c
}

