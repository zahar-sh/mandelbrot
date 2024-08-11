use std::{f64::consts::PI, ops::Deref};

use crate::matrix::VecMatrix;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const BLACK: Rgb = Rgb::new(0, 0, 0);
    pub const WHITE: Rgb = Rgb::new(255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn from_rgba(value: u64) -> Self {
        let r = ((value >> 24) & 0xFF) as u8;
        let g = ((value >> 16) & 0xFF) as u8;
        let b = ((value >> 8) & 0xFF) as u8;
        Self::new(r, g, b)
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Rgb::BLACK
    }
}

pub type RgbImage = VecMatrix<Rgb>;

pub trait Wave {
    type Output;

    fn wave(&self, x: f64) -> Self::Output;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Harmonic {
    pub ampl: f64,
    pub freq: f64,
    pub ang_freq: f64,
    pub phase: f64,
}

impl Harmonic {
    pub fn ang_freq_from_period(period: f64) -> f64 {
        2.0 * PI / period
    }

    pub const fn new(ampl: f64, freq: f64, ang_freq: f64, phase: f64) -> Self {
        Self {
            ampl,
            freq,
            ang_freq,
            phase,
        }
    }
}

impl Default for Harmonic {
    fn default() -> Self {
        Self {
            ampl: 1.0,
            freq: 1.0,
            ang_freq: Harmonic::ang_freq_from_period(1.0),
            phase: 0.0,
        }
    }
}

impl Wave for Harmonic {
    type Output = f64;

    fn wave(&self, x: f64) -> Self::Output {
        self.ampl * (self.freq * self.ang_freq * x + self.phase).sin()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polyharmonic<T>
where
    T: Deref<Target = [Harmonic]>,
{
    pub harmonics: T,
}

impl<T> Polyharmonic<T>
where
    T: Deref<Target = [Harmonic]>,
{
    pub const fn new(harmonics: T) -> Self {
        Self { harmonics }
    }
}

impl<T> Wave for Polyharmonic<T>
where
    T: Deref<Target = [Harmonic]>,
{
    type Output = f64;

    fn wave(&self, x: f64) -> Self::Output {
        self.harmonics.iter().map(|harmonic| harmonic.wave(x)).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RgbWave<R, G, B>
where
    R: Wave<Output = u8>,
    G: Wave<Output = u8>,
    B: Wave<Output = u8>,
{
    pub r: R,
    pub g: G,
    pub b: B,
}

impl<R, G, B> RgbWave<R, G, B>
where
    R: Wave<Output = u8>,
    G: Wave<Output = u8>,
    B: Wave<Output = u8>,
{
    pub const fn new(r: R, g: G, b: B) -> Self {
        Self { r, g, b }
    }
}

impl<R, G, B> Wave for RgbWave<R, G, B>
where
    R: Wave<Output = u8>,
    G: Wave<Output = u8>,
    B: Wave<Output = u8>,
{
    type Output = Rgb;

    fn wave(&self, x: f64) -> Self::Output {
        let r = self.r.wave(x);
        let g = self.g.wave(x);
        let b = self.b.wave(x);
        Rgb::new(r, g, b)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WaveU8<T>
where
    T: Wave<Output = f64>,
{
    pub wave: T,
    pub min: f64,
    pub max: f64,
}

impl<T> WaveU8<T>
where
    T: Wave<Output = f64>,
{
    pub const fn new(wave: T, min: f64, max: f64) -> Self {
        Self { wave, min, max }
    }
}

impl<T> Wave for WaveU8<T>
where
    T: Wave<Output = f64>,
{
    type Output = u8;

    fn wave(&self, x: f64) -> Self::Output {
        let y = self.wave.wave(x);
        let norm_y = ((y - self.min) / (self.max - self.min)).clamp(0.0, 1.0);
        (norm_y * 255.0) as u8
    }
}
