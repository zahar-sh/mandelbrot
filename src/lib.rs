mod mandelbrot;
mod matrix;
mod paint;
mod point;
mod utils;

pub use crate::{
    mandelbrot::*,
    matrix::*,
    paint::*,
    point::*,
    utils::{PipelineError, PipelineResult},
};
