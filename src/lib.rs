mod convert;

pub mod interp;
pub use interp::Interp;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rgb<T = f64> {
    pub r: T,
    pub g: T,
    pub b: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Xyz<T = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Lab<T = f64> {
    pub l: T,
    pub a: T,
    pub b: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Lch<T = f64> {
    pub l: T,
    pub c: T,
    pub h: T,
}
