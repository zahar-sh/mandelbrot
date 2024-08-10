use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use crate::utils::{CrossJoin, TupleMapper};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T, V>
where
    V: Deref<Target = [T]>,
{
    width: u32,
    height: u32,
    data: V,
    _phantom: PhantomData<T>,
}

impl<T, V> Matrix<T, V>
where
    V: Deref<Target = [T]>,
{
    fn from_raw(width: u32, height: u32, data: V) -> Self {
        Self {
            width,
            height,
            data,
            _phantom: PhantomData,
        }
    }

    pub fn try_from_raw(width: u32, height: u32, data: V) -> Result<Self, V> {
        if width as usize * height as usize == data.len() {
            Ok(Self::from_raw(width, height, data))
        } else {
            Err(data)
        }
    }

    pub fn into_raw(self) -> V {
        self.data
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn get(&self, x: u32, y: u32) -> &T {
        match self.data_index_checked(x, y) {
            Some(index) => unsafe { self.data.get_unchecked(index) },
            None => self.index_out_of_bounds(x, y),
        }
    }

    pub fn get_checked(&self, x: u32, y: u32) -> Option<&T> {
        self.data_index_checked(x, y)
            .map(move |index| unsafe { self.data.get_unchecked(index) })
    }

    pub fn indexes(&self) -> impl Iterator<Item = (u32, u32)> {
        (0..self.height).cross_join(0..self.width).flip()
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn pairs(&self) -> impl Iterator<Item = ((u32, u32), &T)> {
        self.indexes().zip(self.values())
    }

    fn data_index(&self, x: u32, y: u32) -> usize {
        y as usize * self.width as usize + x as usize
    }

    fn data_index_checked(&self, x: u32, y: u32) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(self.data_index(x, y))
        } else {
            None
        }
    }

    fn index_out_of_bounds(&self, x: u32, y: u32) -> ! {
        panic!(
            "Matrix index {:?} out of bounds {:?}",
            (x, y),
            (self.width, self.height)
        )
    }
}

impl<T, V> Matrix<T, V>
where
    V: Deref<Target = [T]> + DerefMut,
{
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut T {
        match self.data_index_checked(x, y) {
            Some(index) => unsafe { self.data.get_unchecked_mut(index) },
            None => self.index_out_of_bounds(x, y),
        }
    }

    pub fn get_checked_mut(&mut self, x: u32, y: u32) -> Option<&mut T> {
        self.data_index_checked(x, y)
            .map(move |index| unsafe { self.data.get_unchecked_mut(index) })
    }

    pub fn set(&mut self, x: u32, y: u32, value: T) {
        *self.get_mut(x, y) = value;
    }

    pub fn set_checked(&mut self, x: u32, y: u32, value: T) -> Result<(), T> {
        match self.get_checked_mut(x, y) {
            Some(item) => Ok(*item = value),
            None => Err(value),
        }
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn pairs_mut(&mut self) -> impl Iterator<Item = ((u32, u32), &mut T)> {
        self.indexes().zip(self.values_mut())
    }

    pub fn clear(&mut self)
    where
        T: Default,
    {
        self.data.fill_with(Default::default);
    }
}

impl<T, V> Index<(u32, u32)> for Matrix<T, V>
where
    V: Deref<Target = [T]>,
{
    type Output = T;

    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        self.get(x, y)
    }
}

impl<T, V> IndexMut<(u32, u32)> for Matrix<T, V>
where
    V: Deref<Target = [T]> + DerefMut,
{
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        self.get_mut(x, y)
    }
}

pub type VecMatrix<T> = Matrix<T, Vec<T>>;

impl<T> VecMatrix<T> {
    pub fn new_with<F>(width: u32, height: u32, f: F) -> Self
    where
        F: FnMut() -> T,
    {
        let len = width as usize * height as usize;
        let mut data = Vec::with_capacity(len);
        data.resize_with(len, f);
        Self::from_raw(width, height, data)
    }

    pub fn new(width: u32, height: u32) -> Self
    where
        T: Default,
    {
        Self::new_with(width, height, Default::default)
    }
}
