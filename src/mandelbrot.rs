use std::cmp::Ordering;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub point: Point<f64>,
    pub zoom: f64,
    pub limit: u32,
}

impl Position {
    pub const fn new(point: Point<f64>, zoom: f64, limit: u32) -> Self {
        Self { point, zoom, limit }
    }

    pub fn left(&mut self, offset_scale: f64) {
        self.point.x -= offset_scale / self.zoom;
    }

    pub fn right(&mut self, offset_scale: f64) {
        self.point.x += offset_scale / self.zoom;
    }

    pub fn up(&mut self, offset_scale: f64) {
        self.point.y += offset_scale / self.zoom;
    }

    pub fn down(&mut self, offset_scale: f64) {
        self.point.y -= offset_scale / self.zoom;
    }

    pub fn translate(&mut self, offset_scale: Point<f64>) {
        self.point += offset_scale / self.zoom;
    }

    pub fn change_zoom(&mut self, zoom_scale: f64) {
        self.zoom += self.zoom * zoom_scale;
    }

    pub fn update_limit(&mut self, limit_scale: f64) {
        self.limit = (self.zoom * limit_scale) as u32;
    }

    pub fn clamp_zoom(&mut self, min: f64, max: f64) {
        self.zoom = self.zoom.clamp(min, max);
    }

    pub fn clamp_limit(&mut self, min: u32, max: u32) {
        self.limit = self.limit.clamp(min, max);
    }

    pub fn as_complex(&self) -> Complex64 {
        Complex64::from(self.point)
    }

    pub fn as_complex_with_offset(&self, offset_scale: Point<f64>) -> Complex64 {
        Complex::from(self.point + offset_scale / self.zoom)
    }

    pub fn make_step(
        &mut self,
        to: &Position,
        offset_scale: Point<f64>,
        zoom_scale: f64,
        limit_scale: f64,
    ) -> bool {
        if self.zoom < to.zoom {
            self.make_step_point(to, offset_scale)
                && self.make_step_zoom_and_limit(to, zoom_scale, limit_scale)
        } else {
            self.make_step_zoom_and_limit(to, zoom_scale, limit_scale)
                && self.make_step_point(to, offset_scale)
        }
    }

    fn make_step_point(&mut self, to: &Position, offset_scale: Point<f64>) -> bool {
        let (point, reached) = self.point.get_closer(to.point, offset_scale / self.zoom);
        self.point = point;
        reached
    }

    fn make_step_zoom_and_limit(
        &mut self,
        to: &Position,
        zoom_scale: f64,
        limit_scale: f64,
    ) -> bool {
        let (zoom, reached) = self.zoom.get_closer(to.zoom, self.zoom * zoom_scale);
        self.zoom = zoom;
        if reached {
            self.limit = to.limit;
        } else if self.limit != to.limit {
            let limit_step = ((self.limit as f64 * zoom_scale * limit_scale) as u32).max(1);
            let (limit, _) = self.limit.get_closer(to.limit, limit_step);
            self.limit = limit;
        }
        reached
    }
}

trait GetCloser<T = Self, S = Self> {
    type Output;

    fn get_closer(self, to: T, step: S) -> (Self::Output, bool);
}

impl GetCloser for u32 {
    type Output = Self;

    fn get_closer(self, to: Self, step: Self) -> (Self::Output, bool) {
        match self.cmp(&to) {
            Ordering::Equal => (to, true),
            Ordering::Less => {
                let delta = to - self;
                if delta < step {
                    (to, true)
                } else {
                    (self + step, false)
                }
            }
            Ordering::Greater => {
                let delta = self - to;
                if delta < step {
                    (to, true)
                } else {
                    (self - step, false)
                }
            }
        }
    }
}

impl GetCloser for f64 {
    type Output = Self;

    fn get_closer(self, to: Self, step: Self) -> (Self::Output, bool) {
        match self.partial_cmp(&to) {
            None => (to, true),
            Some(ord) => match ord {
                Ordering::Equal => (to, true),
                Ordering::Less => {
                    let delta = to - self;
                    if delta < step {
                        (to, true)
                    } else {
                        (self + step, false)
                    }
                }
                Ordering::Greater => {
                    let delta = self - to;
                    if delta < step {
                        (to, true)
                    } else {
                        (self - step, false)
                    }
                }
            },
        }
    }
}

impl GetCloser for Point<f64> {
    type Output = Self;

    fn get_closer(self, to: Self, step: Self) -> (Self::Output, bool) {
        let (x, reached_x) = self.x.get_closer(to.x, step.x);
        let (y, reached_y) = self.y.get_closer(to.y, step.y);
        (Self::new(x, y), reached_x && reached_y)
    }
}
