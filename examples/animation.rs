use image::{
    codecs::gif::{GifEncoder, Repeat},
    Frame, ImageError, Rgba, RgbaImage,
};
use mandelbrot::*;
use std::{fs::File, io::BufWriter, path::Path};

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

fn save_animation<P, F>(
    path: P,
    width: u32,
    height: u32,
    pos: &Position,
    mut paint: F,
    period: u32,
    speed: u32,
) -> Result<()>
where
    P: AsRef<Path>,
    F: FnMut(Iteration, u32) -> Rgb + Send + Clone,
{
    let mut matrix = IterationMatrix::new(width, height);
    let mut frame = Frame::new(RgbaImage::new(matrix.width(), matrix.height()));
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    let mut encoder = GifEncoder::new(writer);
    encoder.set_repeat(Repeat::Infinite)?;
    matrix.par_build(&pos, Default::default())?;
    for offset in (0..period).step_by(speed as usize) {
        for (iter, rgba) in matrix.values().zip(frame.buffer_mut().pixels_mut()) {
            let rgb = paint(*iter, offset);
            *rgba = Rgba::from([rgb.r, rgb.g, rgb.b, 255]);
        }
        encoder.encode_frame(frame.clone())?;
    }
    Ok(())
}

fn main() {
    std::fs::create_dir_all("./examples/out").unwrap();
    let path = "./examples/out/animation.gif";
    let (width, height) = (1920, 1080);
    let pos = Positions::JuliaIsland.pos();
    let period = 256;
    let speed = 3;
    let palette = Palette::ElectricBlue;
    let paint = move |iter, offset| match iter {
        Iteration::Finite(iter) => {
            let index = ((iter + offset) % period) as u8;
            let color = palette.get_color(index);
            color
        }
        Iteration::Infinite => Rgb::BLACK,
    };
    save_animation(path, width, height, pos, paint, period, speed).unwrap();
}
