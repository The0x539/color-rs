use crate::{Lab, Lch, Rgb, Xyz};

use core::f64::consts::TAU;
use core::ops::{Add, Div, Mul, Neg, Sub};

impl From<Rgb> for Xyz {
    fn from(rgb: Rgb) -> Self {
        fn f(n: f64) -> f64 {
            if n > 0.04045 {
                n.add(0.055).div(1.055).powf(2.4)
            } else {
                n / 12.92
            }
        }

        let (r, g, b) = (f(rgb.r), f(rgb.g), f(rgb.b));
        let x = r * 0.4124 + g * 0.3576 + b * 0.1805;
        let y = r * 0.2126 + g * 0.7152 + b * 0.0722;
        let z = r * 0.0193 + g * 0.1192 + b * 0.9505;
        Self { x, y, z }
    }
}

impl From<Xyz> for Rgb {
    fn from(xyz: Xyz) -> Self {
        fn f(n: f64) -> f64 {
            if n > 0.0031308 {
                n.powf(2.4_f64.recip()).mul_add(1.055, -0.055)
            } else {
                n * 12.92
            }
        }
        fn f2(n: f64) -> f64 {
            f(n).min(1.0).max(0.0)
        }

        let Xyz { x, y, z } = xyz;
        let r = x * 03.2406 + y * -1.5372 + z * -0.4986;
        let g = x * -0.9689 + y * 01.8758 + z * 00.0415;
        let b = x * 00.0557 + y * -0.2040 + z * 01.0570;
        let (r, g, b) = (f2(r), f2(g), f2(b));
        Self { r, g, b }
    }
}

const REF_X: f64 = 0.95047;
const REF_Y: f64 = 1.00000;
const REF_Z: f64 = 1.08883;

impl From<Xyz> for Lab {
    fn from(xyz: Xyz) -> Self {
        fn f(n: f64) -> f64 {
            if n > 0.008856 {
                n.cbrt()
            } else {
                n.mul_add(7.787, 16.0 / 116.0)
            }
        }

        let (x, y, z) = (f(xyz.x / REF_X), f(xyz.y / REF_Y), f(xyz.z / REF_Z));

        let l = if y > 0.008856 {
            y.mul_add(116.0, -16.0)
        } else {
            y * 903.3
        };
        let a = 500.0 * (x - y);
        let b = 200.0 * (y - z);
        Self { l, a, b }
    }
}

impl From<Lab> for Xyz {
    fn from(lab: Lab) -> Self {
        fn f(n: f64) -> f64 {
            if n.powi(3) > 0.008856 {
                n.powi(3)
            } else {
                n.sub(16.0 / 116.0).div(7.787)
            }
        }

        let y = lab.l.add(16.0).div(116.0);
        let x = lab.a.mul_add(0.002, y);
        let z = lab.b.neg().mul_add(0.005, y);
        let (x, y, z) = (REF_X * f(x), REF_Y * f(y), REF_Z * f(z));
        Self { x, y, z }
    }
}

impl From<Lab> for Lch {
    fn from(lab: Lab) -> Self {
        let Lab { l, a, b } = lab;
        let c = f64::hypot(a, b);
        let mut h = f64::atan2(b, a);
        if h < 0.0 {
            h += TAU;
        }
        Self { l, c, h }
    }
}

impl From<Lch> for Lab {
    fn from(lch: Lch) -> Self {
        let Lch { l, c, h } = lch;
        let (b, a) = h.sin_cos();
        let (a, b) = (a * c, b * c);
        Self { l, a, b }
    }
}

impl<T> Rgb<T> {
    fn convert<U>(self, mut f: impl FnMut(T) -> U) -> Rgb<U> {
        Rgb {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
        }
    }
}

impl Rgb<u8> {
    pub fn to_float64(self) -> Rgb {
        self.convert(|n| (n as f64) / 255.0)
    }
}

impl Rgb {
    pub fn to_int8(self) -> Rgb<u8> {
        self.convert(|n| n.mul(255.0).max(0.0).min(255.0).round() as u8)
    }
    pub fn to_xyz(self) -> Xyz {
        self.into()
    }
    pub fn to_lab(self) -> Lab {
        self.to_xyz().to_lab()
    }
    pub fn to_lch(self) -> Lch {
        self.to_lab().to_lch()
    }
}

impl Xyz {
    pub fn to_rgb(self) -> Rgb {
        self.into()
    }
    pub fn to_lab(self) -> Lab {
        self.into()
    }
    pub fn to_lch(self) -> Lch {
        self.to_lab().to_lch()
    }
}

impl Lab {
    pub fn to_xyz(self) -> Xyz {
        self.into()
    }
    pub fn to_lch(self) -> Lch {
        self.into()
    }
    pub fn to_rgb(self) -> Rgb {
        self.to_xyz().to_rgb()
    }
}

impl Lch {
    pub fn to_lab(self) -> Lab {
        self.into()
    }
    pub fn to_xyz(self) -> Xyz {
        self.to_lab().to_xyz()
    }
    pub fn to_rgb(self) -> Rgb {
        self.to_xyz().to_rgb()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_eq(a: Rgb, b: Rgb) {
        const EPSILON: f64 = 1.0 / 2048.0;
        let dr = (a.r - b.r).abs();
        let dg = (a.g - b.g).abs();
        let db = (a.b - b.b).abs();
        if dr > EPSILON || dg > EPSILON || db > EPSILON {
            panic!("FAIL: {:?} != {:?}", a, b);
        }
    }

    fn test_roundtrip(mut round_trip: impl FnMut(Rgb) -> Rgb) {
        use itertools::iproduct;
        for (r, g, b) in iproduct!(0..=255, 0..=255, 0..=255) {
            let rgb = Rgb { r, g, b }.to_float64();
            test_eq(round_trip(rgb), rgb);
        }
    }

    #[test]
    fn test_xyz_roundtrip() {
        test_roundtrip(|rgb| rgb.to_xyz().to_rgb());
    }

    #[test]
    fn test_lab_roundtrip() {
        test_roundtrip(|rgb| rgb.to_lab().to_rgb());
    }

    #[test]
    fn test_lch_roundtrip() {
        test_roundtrip(|rgb| rgb.to_lch().to_rgb());
    }
}
