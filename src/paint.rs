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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Palette {
    #[default]
    Original,
    Fire,
    BlackAndWhite,
    ElectricBlue,
    Toon,
    Gold,
    ClassicVga,
    Cga1,
    Cga2,
    PrimaryRgb,
    SecondaryCmy,
    Tertiary1,
    Tertiary2,
    Neon,
}

impl Palette {
    pub fn get_color(&self, value: u8) -> Rgb {
        match *self {
            Palette::Original => original(value),
            Palette::Fire => fire(value),
            Palette::BlackAndWhite => black_and_white(value),
            Palette::ElectricBlue => electric_blue(value),
            Palette::Toon => toon(value),
            Palette::Gold => gold(value),
            Palette::ClassicVga => classic_vga(value),
            Palette::Cga1 => cga1(value),
            Palette::Cga2 => cga2(value),
            Palette::PrimaryRgb => primary_rgb(value),
            Palette::SecondaryCmy => secondary_cmy(value),
            Palette::Tertiary1 => tertiary1(value),
            Palette::Tertiary2 => tertiary2(value),
            Palette::Neon => neon(value),
        }
    }
}

fn original(num: u8) -> Rgb {
    if num < 32 {
        Rgb::new(num * 8, num * 8, 127 - num * 4)
    } else if num < 128 {
        Rgb::new(255, 255 - (num - 32) * 8 / 3, (num - 32) * 4 / 3)
    } else if num < 192 {
        Rgb::new(
            255 - (num - 128) * 4,
            0 + (num - 128) * 3,
            127 - (num - 128),
        )
    } else {
        Rgb::new(0, 192 - (num - 192) * 3, 64 + (num - 192))
    }
}

fn fire(num: u8) -> Rgb {
    if num < 64 {
        Rgb::new(num * 4, 0, 0)
    } else if num < 128 {
        Rgb::new(255, (num - 64) * 2, 0)
    } else if num < 192 {
        Rgb::new(255, 128 - ((num - 128) * 2), 0)
    } else {
        Rgb::new(255 - (num - 192) * 4, 0, 0)
    }
}

fn black_and_white(num: u8) -> Rgb {
    if num < 128 {
        Rgb::new(255 - num * 2, 255 - num * 2, 255 - num * 2)
    } else {
        Rgb::new((num - 128) * 2, (num - 128) * 2, (num - 128) * 2)
    }
}

fn electric_blue(num: u8) -> Rgb {
    if num < 32 {
        Rgb::new(0, 0, num * 4)
    } else if num < 64 {
        Rgb::new((num - 32) * 8, (num - 32) * 8, 127 + (num - 32) * 4)
    } else if num < 96 {
        Rgb::new(
            255 - (num - 64) * 8,
            255 - (num - 64) * 8,
            255 - (num - 64) * 4,
        )
    } else if num < 128 {
        Rgb::new(0, 0, 127 - (num - 96) * 4)
    } else if num < 192 {
        Rgb::new(0, 0, num - 128)
    } else {
        Rgb::new(0, 0, 63 - (num - 192))
    }
}

fn toon(num: u8) -> Rgb {
    let num = num % 4;
    if num == 0 {
        Rgb::new(100, 20, 200)
    } else if num == 1 {
        Rgb::new(220, 112, 0)
    } else if num == 2 {
        Rgb::new(230, 120, 0)
    } else {
        Rgb::new(255, 128, 0)
    }
}

fn gold(num: u8) -> Rgb {
    if num < 32 {
        Rgb::new(
            54 + ((num) * (224 - 54) / 32),
            11 + ((num) * (115 - 11) / 32),
            2 + ((num) * (10 - 2) / 32),
        )
    } else if num < 64 {
        Rgb::new(
            224 + ((num - 32) * (255 - 224) / 32),
            115 + ((num - 32) * (192 - 115) / 32),
            10 + ((num - 32) * (49 - 10) / 32),
        )
    } else if num < 192 {
        Rgb::new(
            255,
            192 + ((num - 64) * (255 - 192) / 128),
            49 + ((num - 64) * (166 - 49) / 128),
        )
    } else if num < 224 {
        Rgb::new(
            255,
            255 - ((num - 192) * (255 - 192) / 32),
            166 - ((num - 192) * (166 - 49) / 32),
        )
    } else {
        Rgb::new(
            255 - ((num - 224) * (255 - 54) / 32),
            192 - ((num - 224) * (192 - 11) / 32),
            49 - ((num - 224) * (49 - 2) / 32),
        )
    }
}

fn classic_vga(num: u8) -> Rgb {
    let index = num as usize % VGA.len();
    let vga = VGA[index];
    Rgb::from_rgba(vga)
}

fn cga1(num: u8) -> Rgb {
    let num = num % 4;
    if num == 0 {
        Rgb::from_rgba(0)
    } else if num == 1 {
        Rgb::from_rgba(1442840320)
    } else if num == 2 {
        Rgb::from_rgba(4283825920)
    } else {
        Rgb::from_rgba(4294967040)
    }
}

fn cga2(num: u8) -> Rgb {
    let num = num % 4;
    if num == 0 {
        Rgb::from_rgba(0)
    } else if num == 1 {
        Rgb::from_rgba(1442796800)
    } else if num == 2 {
        Rgb::from_rgba(4283782400)
    } else {
        Rgb::from_rgba(4294923520)
    }
}

fn primary_rgb(num: u8) -> Rgb {
    if num < 85 {
        Rgb::new(255 - num * 3, 0 + num * 3, 0)
    } else if num < 170 {
        Rgb::new(0, 255 - (num - 85) * 3, 0 + (num - 85) * 3)
    } else {
        Rgb::new(0 + (num - 170) * 3, 0, 255 - (num - 170) * 3)
    }
}

fn secondary_cmy(num: u8) -> Rgb {
    if num < 85 {
        Rgb::new(0 + num * 3, 255 - num * 3, 255)
    } else if num < 170 {
        Rgb::new(255, 0 + (num - 85) * 3, 255 - (num - 85) * 3)
    } else {
        Rgb::new(255 - (num - 170) * 3, 255, 0 + (num - 170) * 3)
    }
}

fn tertiary1(num: u8) -> Rgb {
    if num < 85 {
        Rgb::new(255 - num * 3 / 2, 127 - num * 3 / 2, 0 + num * 3)
    } else if num < 170 {
        Rgb::new(
            127 - (num - 85) * 3 / 2,
            0 + (num - 85) * 3,
            255 - (num - 85) * 3 / 2,
        )
    } else {
        Rgb::new(
            0 + (num - 170) * 3,
            255 - (num - 170) * 3 / 2,
            127 - (num - 170) * 3 / 2,
        )
    }
}

fn tertiary2(num: u8) -> Rgb {
    if num < 85 {
        Rgb::new(255 - num * 3, 0 + num * 3 / 2, 127 + num * 3 / 2)
    } else if num < 170 {
        Rgb::new(
            0 + (num - 85) * 3 / 2,
            127 + (num - 85) * 3 / 2,
            255 - (num - 85) * 3,
        )
    } else {
        Rgb::new(
            127 + (num - 170) * 3 / 2,
            255 - (num - 170) * 3,
            0 + (num - 170) * 3 / 2,
        )
    }
}

fn neon(num: u8) -> Rgb {
    if num < 32 {
        Rgb::new(num * 4, 0, num * 8)
    } else if num < 64 {
        Rgb::new(124 - (num - 32) * 4, 0, 248 - (num - 32) * 8)
    } else if num < 96 {
        Rgb::new((num - 64) * 8, (num - 64) * 4, 0)
    } else if num < 128 {
        Rgb::new(248 - (num - 96) * 8, 124 - (num - 96) * 4, 0)
    } else if num < 160 {
        Rgb::new(0, (num - 128) * 4, (num - 128) * 8)
    } else if num < 192 {
        Rgb::new(0, 124 - (num - 160) * 4, 248 - (num - 160) * 8)
    } else if num < 224 {
        Rgb::new((num - 192) * 4, (num - 192) * 8, (num - 192) * 4)
    } else {
        Rgb::new(
            124 - (num - 224) * 4,
            248 - (num - 224) * 8,
            124 - (num - 224) * 4,
        )
    }
}

const VGA: &[u64] = &[
    0, 43520, 11141120, 11184640, 2852126720, 2852170240, 2857697280, 2863311360, 1431655680,
    1431699200, 1442796800, 1442840320, 4283782400, 4283825920, 4294923520, 4294967040, 0,
    336860160, 538976256, 741092352, 943208448, 1162167552, 1364283648, 1633771776, 1903259904,
    2189591040, 2459079168, 2728567296, 3065427456, 3419130624, 3823362816, 4294967040, 65280,
    1090584320, 2097217280, 3187736320, 4278255360, 4278238720, 4278222080, 4278206720, 4278190080,
    4282449920, 4286382080, 4290641920, 4294901760, 3204382720, 2113863680, 1107230720, 16711680,
    16728320, 16743680, 16760320, 16776960, 12517120, 8257280, 4325120, 2105409280, 2659057408,
    3195928320, 3749576448, 4286447360, 4286439168, 4286430720, 4286422528, 4286414080, 4288576768,
    4290673920, 4292836608, 4294933760, 3758062848, 3204414720, 2667543808, 2113895680, 2113904128,
    2113912320, 2113920768, 2113928960, 2111831808, 2109669120, 2107571968, 3065446144, 3350658816,
    3686203136, 3954638592, 4290182912, 4290177792, 4290173696, 4290168576, 4290164224, 4291278336,
    4292589056, 4293637632, 4294948352, 3959404032, 3690968576, 3355424256, 3070211584, 3070215936,
    3070221056, 3070225152, 3070230272, 3068919552, 3067870976, 3066560256, 28928, 469790976,
    939553024, 1426092288, 1895854336, 1895847168, 1895839744, 1895832576, 1895825408, 1897660416,
    1899495424, 1901395968, 1903230976, 1433468928, 946929664, 477167616, 7405568, 7412736,
    7419904, 7427328, 7434496, 5599488, 3698944, 1863936, 943223040, 1161326848, 1429762304,
    1631088896, 1899524352, 1899520256, 1899517184, 1899513088, 1899509760, 1900361728, 1901410304,
    1902196736, 1903245312, 1634809856, 1433483264, 1165047808, 946944000, 946947328, 946951424,
    946954496, 946958592, 945910016, 945123584, 944075008, 1364291840, 1498509568, 1632727296,
    1766945024, 1901162752, 1901160704, 1901158656, 1901156608, 1901154560, 1901678848, 1902203136,
    1902727424, 1903251712, 1769033984, 1634816256, 1500598528, 1366380800, 1366382848, 1366384896,
    1366386944, 1366388992, 1365864704, 1365340416, 1364816128, 16640, 268452096, 536887552,
    805323008, 1090535680, 1090531328, 1090527232, 1090523136, 1090519040, 1091567616, 1092616192,
    1093664768, 1094778880, 809566208, 541130752, 272695296, 4259840, 4263936, 4268032, 4272128,
    4276480, 3162368, 2113792, 1065216, 538984704, 673202432, 807420160, 941637888, 1092632832,
    1092630528, 1092628480, 1092626432, 1092624384, 1093148672, 1093672960, 1094197248, 1094787072,
    943792128, 809574400, 675356672, 541138944, 541140992, 541143040, 541145088, 541147392,
    540557568, 540033280, 539508992, 741097728, 808206592, 875315456, 1009533184, 1093419264,
    1093417984, 1093415936, 1093414912, 1093413888, 1093676032, 1093938176, 1094462464, 1094790144,
    1010904064, 876686336, 809577472, 742468608, 742469632, 742470656, 742472704, 742473984,
    742146304, 741622016, 741359872, 0, 0, 0, 0, 0, 0, 0, 0,
];
