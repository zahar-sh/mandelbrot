use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

use num::{complex::Complex64, Complex};

use crate::{
    matrix::{Matrix, VecMatrix},
    point::Point,
    utils::{pipeline, CrossJoin, Duplicate, PipelineResult, TupleMapper},
};

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

#[derive(Debug, Clone, PartialEq)]
pub struct PositionController {
    pub pos: Position,
    pub step: Point<f64>,
    pub zoom_scale: f64,
    pub min_zoom: f64,
    pub max_zoom: f64,
    pub limit_scale: f64,
    pub min_limit: u32,
    pub max_limit: u32,
}

impl PositionController {
    pub fn left(&mut self) {
        self.pos.left(self.step.x);
    }

    pub fn right(&mut self) {
        self.pos.right(self.step.x);
    }

    pub fn up(&mut self) {
        self.pos.up(self.step.y);
    }

    pub fn down(&mut self) {
        self.pos.down(self.step.y);
    }

    pub fn translate(&mut self, offset_scale: Point<f64>) {
        self.pos.translate(offset_scale);
    }

    pub fn change_zoom(&mut self, zoom_scale: f64) {
        self.pos.change_zoom(zoom_scale);
        self.clamp_zoom();
    }

    pub fn increase_zoom(&mut self) {
        self.change_zoom(self.zoom_scale);
    }

    pub fn decrease_zoom(&mut self) {
        self.change_zoom(-self.zoom_scale);
    }

    pub fn update_limit(&mut self) {
        self.pos.update_limit(self.limit_scale);
        self.clamp_limit();
    }

    pub fn clamp_zoom(&mut self) {
        self.pos.clamp_zoom(self.min_zoom, self.max_zoom);
    }

    pub fn clamp_limit(&mut self) {
        self.pos.clamp_limit(self.min_limit, self.max_limit);
    }

    pub fn make_step(&mut self, to: &Position) -> bool {
        self.pos
            .make_step(to, self.step, self.zoom_scale, self.limit_scale)
    }
}

impl Default for PositionController {
    fn default() -> Self {
        Self {
            pos: Position::new(Point::new(-1.34228, 0.0), 300.0, 200),
            step: Point::new(10.0, 10.0),
            zoom_scale: 0.2,
            min_zoom: 50.0,
            max_zoom: 4500000000000000.0,
            limit_scale: 0.25,
            min_limit: 150,
            max_limit: 1500,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuildMandelbrotSetOptions {
    pub viewport_offset_scale: Option<Point<f64>>,
    pub smooth: Option<Point<u32>>,
}

impl BuildMandelbrotSetOptions {
    pub fn viewport_offset_scale(mut self, viewport_offset_scale: Point<f64>) -> Self {
        self.viewport_offset_scale = Some(viewport_offset_scale);
        self
    }

    pub fn smooth(mut self, smooth: Point<u32>) -> Self {
        self.smooth = Some(smooth);
        self
    }
}

pub trait MandelbrotSet {
    fn build(self, pos: &Position, options: BuildMandelbrotSetOptions);
}

pub trait MandelbrotSetImage<T> {
    fn build_image<F>(self, pos: &Position, convert: F, options: BuildMandelbrotSetOptions)
    where
        F: FnMut(Iteration) -> T;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParallelBuildMandelbrotSetOptions {
    pub viewport_offset_scale: Option<Point<f64>>,
    pub smooth: Option<Point<u32>>,
    pub workers: Option<u32>,
}

impl ParallelBuildMandelbrotSetOptions {
    pub fn viewport_offset_scale(mut self, viewport_offset_scale: Point<f64>) -> Self {
        self.viewport_offset_scale = Some(viewport_offset_scale);
        self
    }

    pub fn smooth(mut self, smooth: Point<u32>) -> Self {
        self.smooth = Some(smooth);
        self
    }

    pub fn workers(mut self, workers: u32) -> Self {
        self.workers = Some(workers);
        self
    }
}

pub trait ParallelMandelbrotSet {
    fn par_build(
        self,
        pos: &Position,
        options: ParallelBuildMandelbrotSetOptions,
    ) -> PipelineResult<()>;
}

pub trait ParallelMandelbrotSetImage<T> {
    fn par_build_image<F>(
        self,
        pos: &Position,
        convert: F,
        options: ParallelBuildMandelbrotSetOptions,
    ) -> PipelineResult<()>
    where
        F: FnMut(Iteration) -> T + Send + Clone;
}

impl<T> MandelbrotSet for T
where
    T: MandelbrotSetImage<Iteration>,
{
    fn build(self, pos: &Position, options: BuildMandelbrotSetOptions) {
        self.build_image(pos, |iter| iter, options)
    }
}

impl<T> ParallelMandelbrotSet for T
where
    T: ParallelMandelbrotSetImage<Iteration>,
{
    fn par_build(
        self,
        pos: &Position,
        options: ParallelBuildMandelbrotSetOptions,
    ) -> PipelineResult<()> {
        self.par_build_image(pos, |iter| iter, options)
    }
}

impl<'a, T, V> MandelbrotSetImage<T> for &'a mut Matrix<T, V>
where
    T: Clone,
    V: Deref<Target = [T]> + DerefMut,
{
    fn build_image<F>(self, pos: &Position, mut convert: F, options: BuildMandelbrotSetOptions)
    where
        F: FnMut(Iteration) -> T,
    {
        let BuildMandelbrotSetOptions {
            viewport_offset_scale,
            smooth,
        } = options;
        let (width, height) = self.size();
        let point_offset = get_point_offset(width, height, viewport_offset_scale, smooth);
        let mut transform_point_to_item = move |point| {
            let point = point + point_offset;
            let complex = pos.as_complex_with_offset(point);
            let iter = complex.compute_iterations(pos.limit);
            let item = convert(iter);
            item
        };
        let transform_index_to_item = move |index| {
            let point = Point::from(index).transform(|v| v as f64);
            let item = transform_point_to_item(point);
            item
        };
        match smooth {
            Some(smooth) => {
                let indexes_groups = index_groups(width, height, smooth.x, smooth.y);
                let item_indexes_pairs = indexes_groups.map_first(transform_index_to_item);
                for (item, indexes) in item_indexes_pairs {
                    for (x, y) in indexes {
                        self.set(x, y, item.clone());
                    }
                }
            }
            None => {
                for (item, dest) in self.pairs_mut().map_first(transform_index_to_item) {
                    *dest = item;
                }
            }
        }
    }
}

impl<'a, T, V> ParallelMandelbrotSetImage<T> for &'a mut Matrix<T, V>
where
    T: Send + Clone,
    V: Deref<Target = [T]> + DerefMut,
{
    fn par_build_image<F>(
        self,
        pos: &Position,
        mut convert: F,
        options: ParallelBuildMandelbrotSetOptions,
    ) -> PipelineResult<()>
    where
        F: FnMut(Iteration) -> T + Send + Clone,
    {
        let ParallelBuildMandelbrotSetOptions {
            viewport_offset_scale,
            smooth,
            workers,
        } = options;
        let (width, height) = self.size();
        let point_offset = get_point_offset(width, height, viewport_offset_scale, smooth);
        let mut transform_point_to_item = move |point| {
            let point = point + point_offset;
            let complex = pos.as_complex_with_offset(point);
            let iter = complex.compute_iterations(pos.limit);
            let item = convert(iter);
            item
        };
        let mut transform_index_to_item = move |index| {
            let point = Point::from(index).transform(|v| v as f64);
            let item = transform_point_to_item(point);
            item
        };
        match smooth {
            Some(smooth) => pipeline(
                index_groups(width, height, smooth.x, smooth.y),
                move |(index, indexes)| {
                    let item = transform_index_to_item(index);
                    (item, indexes)
                },
                move |recv| {
                    for (item, indexes) in recv.into_iter() {
                        for (x, y) in indexes {
                            self.set(x, y, item.clone());
                        }
                    }
                },
                workers,
            ),
            None => pipeline(
                self.pairs_mut(),
                move |(index, dest)| {
                    let item = transform_index_to_item(index);
                    (item, dest)
                },
                move |recv| {
                    for (item, dest) in recv.into_iter() {
                        *dest = item;
                    }
                },
                workers,
            ),
        }
    }
}

fn get_point_offset(
    width: u32,
    height: u32,
    viewport_offset_scale: Option<Point<f64>>,
    smooth: Option<Point<u32>>,
) -> Point<f64> {
    let viewport_offset = Point::new(width, height).transform(|v| v as f64)
        * -viewport_offset_scale.unwrap_or(Point::new(0.5, 0.5));
    let rect_offset = smooth
        .map(|step| (step / 2).transform(|v| v as f64))
        .unwrap_or_default();
    viewport_offset + rect_offset
}

fn index_groups(
    width: u32,
    height: u32,
    step_x: u32,
    step_y: u32,
) -> impl Iterator<Item = ((u32, u32), impl Iterator<Item = (u32, u32)>)> {
    let indexes = indexes_step_by(width, height, step_x, step_y);
    let groups = indexes.duplicate().map_second(move |(x, y)| {
        let rect = (0..step_y).cross_join(0..step_x).flip();
        let indexes = rect
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter(move |&(x, y)| x < width && y < height);
        indexes
    });
    groups
}

fn indexes_step_by(
    width: u32,
    height: u32,
    step_x: u32,
    step_y: u32,
) -> impl Iterator<Item = (u32, u32)> {
    (0..height)
        .step_by(step_y as usize)
        .cross_join((0..width).step_by(step_x as usize))
        .flip()
}

pub type IterationMatrix = VecMatrix<Iteration>;
