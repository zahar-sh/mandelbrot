pub trait CrossJoin
where
    Self: Iterator + Sized,
    Self::Item: Clone,
{
    fn cross_join<U>(self, other: U) -> impl Iterator<Item = (Self::Item, U::Item)>
    where
        U: Iterator + Clone,
    {
        self.flat_map(move |item| std::iter::repeat(item).zip(other.clone()))
    }
}

impl<T> CrossJoin for T
where
    T: Iterator + Sized,
    T::Item: Clone,
{
}

pub trait Duplicate
where
    Self: Iterator + Sized,
    Self::Item: Clone,
{
    fn duplicate(self) -> impl Iterator<Item = (Self::Item, Self::Item)> {
        self.map(|item| (item.clone(), item))
    }
}

impl<T> Duplicate for T
where
    T: Iterator + Sized,
    T::Item: Clone,
{
}

pub trait TupleMapper<A, B>
where
    Self: Iterator<Item = (A, B)> + Sized,
{
    fn flip(self) -> impl Iterator<Item = (B, A)> {
        self.map(|(a, b)| (b, a))
    }

    fn map_first<R, F>(self, mut f: F) -> impl Iterator<Item = (R, B)>
    where
        F: FnMut(A) -> R,
    {
        self.map(move |(a, b)| (f(a), b))
    }

    fn map_second<R, F>(self, mut f: F) -> impl Iterator<Item = (A, R)>
    where
        F: FnMut(B) -> R,
    {
        self.map(move |(a, b)| (a, f(b)))
    }
}

impl<T, A, B> TupleMapper<A, B> for T where T: Iterator<Item = (A, B)> + Sized {}
