use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn splat(value: T) -> Self
    where
        T: Clone,
    {
        Self::new(value.clone(), value)
    }

    pub fn transform<R, F>(self, mut f: F) -> Point<R>
    where
        F: FnMut(T) -> R,
    {
        let x = f(self.x);
        let y = f(self.y);
        Point::new(x, y)
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T> Into<(T, T)> for Point<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> Neg for Point<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Mul for Point<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> Div for Point<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T> Rem for Point<T>
where
    T: Rem<Output = T>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.x % rhs.x, self.y % rhs.y)
    }
}

impl<T> AddAssign for Point<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign for Point<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> MulAssign for Point<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> DivAssign for Point<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T> RemAssign for Point<T>
where
    T: RemAssign,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

impl<T> Add<T> for Point<T>
where
    T: Clone,
    Self: Add<Output = Self>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        self + Self::splat(rhs)
    }
}

impl<T> Sub<T> for Point<T>
where
    T: Clone,
    Self: Sub<Output = Self>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        self - Self::splat(rhs)
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Clone,
    Self: Mul<Output = Self>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self * Self::splat(rhs)
    }
}

impl<T> Div<T> for Point<T>
where
    T: Clone,
    Self: Div<Output = Self>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self / Self::splat(rhs)
    }
}

impl<T> Rem<T> for Point<T>
where
    T: Clone,
    Self: Rem<Output = Self>,
{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        self % Self::splat(rhs)
    }
}

impl<T> AddAssign<T> for Point<T>
where
    T: Clone,
    Self: AddAssign,
{
    fn add_assign(&mut self, rhs: T) {
        *self += Self::splat(rhs);
    }
}

impl<T> SubAssign<T> for Point<T>
where
    T: Clone,
    Self: SubAssign,
{
    fn sub_assign(&mut self, rhs: T) {
        *self -= Self::splat(rhs);
    }
}

impl<T> MulAssign<T> for Point<T>
where
    T: Clone,
    Self: MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        *self *= Self::splat(rhs);
    }
}

impl<T> DivAssign<T> for Point<T>
where
    T: Clone,
    Self: DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        *self /= Self::splat(rhs);
    }
}

impl<T> RemAssign<T> for Point<T>
where
    T: Clone,
    Self: RemAssign,
{
    fn rem_assign(&mut self, rhs: T) {
        *self %= Self::splat(rhs);
    }
}
