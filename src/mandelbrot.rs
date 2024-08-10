use num::{complex::Complex64, Complex};

use crate::point::Point;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Iteration {
    Finite(u32),
    #[default]
    Infinite,
}

impl From<Iteration> for Option<u32> {
    fn from(value: Iteration) -> Self {
        match value {
            Iteration::Finite(iter) => Some(iter),
            Iteration::Infinite => None,
        }
    }
}

pub trait MandelbrotComplex {
    fn compute_iterations(&self, limit: u32) -> Iteration;
}

impl MandelbrotComplex for Complex64 {
    fn compute_iterations(&self, limit: u32) -> Iteration {
        let Self { re, im } = *self;
        if re > -0.5 && re < 0.25 && im > -0.5 && im < 0.5 {
            return Iteration::Infinite;
        }
        let mut z_re = re;
        let mut z_im = im;
        for i in 0..limit {
            let sq_re = z_re * z_re;
            let sq_im = z_im * z_im;
            if (sq_re + sq_im) > 4.0 {
                return Iteration::Finite(i);
            }
            z_im = 2.0 * z_re * z_im + im;
            z_re = sq_re - sq_im + re;
        }
        return Iteration::Infinite;
    }
}

impl<T> From<Point<T>> for Complex<T> {
    fn from(value: Point<T>) -> Self {
        Self::new(value.x, value.y)
    }
}
