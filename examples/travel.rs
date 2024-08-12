use image::{
    codecs::gif::{GifEncoder, Repeat},
    Frame, ImageError, Rgba, RgbaImage,
};
use mandelbrot::*;
use std::{f64::consts::*, fs::File, io::BufWriter, path::Path};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Pipeline(PipelineError),
    Image(ImageError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<PipelineError> for Error {
    fn from(value: PipelineError) -> Self {
        Self::Pipeline(value)
    }
}

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        Self::Image(value)
    }
}

type Result<T> = std::result::Result<T, Error>;

type PolyharmonicWaveU8 = WaveU8<Polyharmonic<Vec<Harmonic>>>;

fn wave_1(ang_freq: f64) -> PolyharmonicWaveU8 {
    WaveU8::new(
        Polyharmonic::new(vec![
            Harmonic::new(1.0, 1.0, ang_freq, 0.0),
            Harmonic::new(1.0, 2.0, ang_freq, FRAC_PI_4),
            Harmonic::new(1.0, 3.0, ang_freq, FRAC_PI_6),
            Harmonic::new(1.0, 4.0, ang_freq, 2.0 * PI),
            Harmonic::new(1.0, 5.0, ang_freq, PI),
        ]),
        -3.1,
        2.7,
    )
}

fn wave_2(ang_freq: f64) -> PolyharmonicWaveU8 {
    WaveU8::new(
        Polyharmonic::new(vec![
            Harmonic::new(1.0, 1.0, ang_freq, PI / 9.0),
            Harmonic::new(1.0, 2.0, ang_freq, FRAC_PI_4),
            Harmonic::new(1.0, 3.0, ang_freq, FRAC_PI_3),
            Harmonic::new(1.0, 4.0, ang_freq, FRAC_PI_6),
            Harmonic::new(1.0, 5.0, ang_freq, 0.0),
        ]),
        -2.5,
        4.5,
    )
}

fn wave_3(ang_freq: f64) -> PolyharmonicWaveU8 {
    WaveU8::new(
        Polyharmonic::new(vec![
            Harmonic::new(1.0, 1.0, ang_freq, FRAC_PI_4),
            Harmonic::new(1.0, 2.0, ang_freq, 3.0 * FRAC_PI_4),
            Harmonic::new(1.0, 3.0, ang_freq, 2.0 * FRAC_PI_3),
            Harmonic::new(1.0, 4.0, ang_freq, FRAC_PI_2),
            Harmonic::new(1.0, 5.0, ang_freq, FRAC_PI_3),
        ]),
        -2.6,
        4.2,
    )
}

fn wave_4(ang_freq: f64) -> PolyharmonicWaveU8 {
    WaveU8::new(
        Polyharmonic::new(vec![
            Harmonic::new(1.0, 1.0, ang_freq, FRAC_PI_2),
            Harmonic::new(1.0, 2.0, ang_freq, 0.0),
            Harmonic::new(1.0, 3.0, ang_freq, FRAC_PI_4),
            Harmonic::new(1.0, 4.0, ang_freq, FRAC_PI_3),
            Harmonic::new(1.0, 5.0, ang_freq, FRAC_PI_6),
        ]),
        -2.5,
        4.4,
    )
}

fn wave_5(ang_freq: f64) -> PolyharmonicWaveU8 {
    WaveU8::new(
        Polyharmonic::new(vec![
            Harmonic::new(1.0, 1.0, ang_freq, PI),
            Harmonic::new(1.0, 2.0, ang_freq, FRAC_PI_4),
            Harmonic::new(1.0, 3.0, ang_freq, 0.0),
            Harmonic::new(1.0, 4.0, ang_freq, 3.0 * FRAC_PI_4),
            Harmonic::new(1.0, 5.0, ang_freq, FRAC_PI_2),
        ]),
        -4.0,
        2.4,
    )
}

fn create_table<T>(len: u32, scale: f64, wave: T) -> Vec<Rgb>
where
    T: Wave<Output = Rgb>,
{
    (0..len)
        .map(move |x| x as f64 * scale)
        .map(move |x| wave.wave(x))
        .collect()
}

fn save_travel_animation<P, F>(
    path: P,
    width: u32,
    height: u32,
    start_pos: &Position,
    end_pos: &Position,
    paint: F,
) -> Result<()>
where
    P: AsRef<Path>,
    F: FnMut(Iteration) -> Rgb + Send + Clone,
{
    let mut controller = PositionController {
        pos: start_pos.clone(),
        ..Default::default()
    };
    let mut image = RgbImage::new(width, height);
    let mut frame = Frame::new(RgbaImage::new(image.width(), image.height()));
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    let mut encoder = GifEncoder::new(writer);
    encoder.set_repeat(Repeat::Finite(0))?;
    while !controller.make_step(end_pos) {
        image.par_build_image(&controller.pos, paint.clone(), Default::default())?;
        for (rgb, rgba) in image.values().zip(frame.buffer_mut().pixels_mut()) {
            *rgba = Rgba::from([rgb.r, rgb.g, rgb.b, 255]);
        }
        encoder.encode_frame(frame.clone())?;
    }
    Ok(())
}

fn main() {
    std::fs::create_dir_all("./examples/out").unwrap();
    let path = "./examples/out/travel.gif";
    let (width, height) = (1920, 1080);
    let (from, to) = (Positions::Home.pos(), Positions::JuliaIsland.pos());
    let period = 1024;
    let color_scale = 16;
    let ang_freq = Harmonic::ang_freq_from_period((period - 1) as f64);
    let wave = RgbWave::new(wave_2(ang_freq), wave_3(ang_freq), wave_4(ang_freq));
    let table = create_table(period / color_scale, color_scale as f64, wave);
    let paint = move |iter| match iter {
        Iteration::Finite(iter) => {
            let index = iter as usize % table.len();
            let color = table[index];
            color
        }
        Iteration::Infinite => Rgb::BLACK,
    };
    save_travel_animation(path, width, height, from, to, paint).unwrap();
}
