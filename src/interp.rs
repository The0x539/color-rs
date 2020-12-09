use crate::{Lab, Lch, Rgb, Xyz};

use core::f64::consts::{PI, TAU};

pub trait Interp<T> {
    fn interp(self, v0: T, v1: T) -> T;
}

impl Interp<f64> for f64 {
    fn interp(self, v0: f64, v1: f64) -> f64 {
        let (t, u) = (self, 1.0 - self);
        v0 * u + v1 * t
    }
}

macro_rules! interp_impl {
    ($(impl Interp<$t:ident> { $($field:ident),*$(,)? })*) => {
        $(impl<T, X: Copy + Interp<T>> Interp<$t<T>> for X {
            fn interp(self, v0: $t<T>, v1: $t<T>) -> $t<T> {
                $t {
                    $($field: self.interp(v0.$field, v1.$field)),*
                }
            }
        })*
    };
}

interp_impl! {
    impl Interp<Rgb> { r, g, b }
    impl Interp<Xyz> { x, y, z }
    impl Interp<Lab> { l, a, b }
}

impl<X> Interp<Lch> for X
where
    X: Copy + Interp<f64>,
{
    fn interp(self, v0: Lch, v1: Lch) -> Lch {
        let l = self.interp(v0.l, v1.l);
        let c = self.interp(v0.c, v1.c);
        let (mut h0, mut h1) = (v0.h, v1.h);
        if h1 - h0 >= PI {
            h1 -= TAU;
        } else if h0 - h1 >= PI {
            h0 -= TAU;
        }
        let mut h = self.interp(h0, h1);
        if h < 0.0 {
            h += TAU;
        }
        Lch { l, c, h }
    }
}
