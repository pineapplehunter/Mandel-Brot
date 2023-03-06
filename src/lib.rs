use std::ops::{Add, Mul, Neg};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    re: f64,
    im: f64,
    scale: f64,
) -> Result<(), JsValue> {
    // The real workhorse of this algorithm, generating pixel data
    let c = Complex { re, im };
    let mut data = mandelbrot_set(width, height, c, scale);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn mandelbrot_set(width: u32, height: u32, c: Complex, scale: f64) -> Vec<u8> {
    let width = width as i32;
    let height = height as i32;
    let scale = (10.0 as f64).powf(-scale);

    let mut data = Vec::with_capacity((width * height * 4) as usize);

    let scale_x = 1. / width as f64;
    let scale_y = 1. / height as f64;

    for y in 0..width {
        for x in 0..height {
            let c = Complex {
                re: (x - width / 2) as f64 * scale_x,
                im: (y - height / 2) as f64 * scale_y,
            } * scale
                + c;
            let iter_index = get_iter_index(c);
            data.push((iter_index / 4) as u8);
            data.push((iter_index / 2) as u8);
            data.push(iter_index as u8);
            data.push(255);
        }
    }

    data
}

fn get_iter_index(c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = Complex::zero();
    while iter_index < 1000 {
        if z.norm() > 2.0 {
            break;
        }
        z = z * z + c;
        iter_index += 1;
    }
    iter_index
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn norm(&self) -> f64 {
        ((self.re * self.re) + (self.im * self.im)).sqrt()
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl Complex {
    pub fn zero() -> Complex {
        Complex { re: 0., im: 0. }
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, rhs: f64) -> Self::Output {
        Complex {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}
