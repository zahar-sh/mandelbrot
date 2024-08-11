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

pub type PipelineError = Box<dyn std::any::Any + Send>;

pub type PipelineResult<T> = Result<T, PipelineError>;

pub(crate) fn pipeline<T, U, R, I, F, A>(
    items: I,
    map: F,
    mut action: A,
    workers: Option<u32>,
) -> PipelineResult<R>
where
    T: Send,
    I: Iterator<Item = T> + Send,
    U: Send,
    F: FnMut(T) -> U + Send + Clone,
    A: FnMut(crossbeam::channel::Receiver<U>) -> R,
{
    let workers = workers
        .map(|v| v as usize)
        .unwrap_or_else(|| num_cpus::get())
        .saturating_sub(1)
        .max(1);
    let channel_cap = workers * 2;
    let (item_snd, item_recv) = crossbeam::channel::bounded(channel_cap);
    let (result_snd, result_recv) = crossbeam::channel::bounded(channel_cap);
    let result = crossbeam::scope(move |s| {
        s.spawn(move |_| {
            for item in items {
                item_snd.send(item).unwrap();
            }
        });
        for _ in 0..workers {
            let item_recv = item_recv.clone();
            let result_snd = result_snd.clone();
            let mut map = map.clone();
            s.spawn(move |_| {
                for item in item_recv {
                    let result = map(item);
                    result_snd.send(result).unwrap();
                }
            });
        }
        drop(result_snd);
        let result = action(result_recv);
        result
    });
    result
}
