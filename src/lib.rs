use std::marker::PhantomData;

pub struct DelayCoordinates {
    dimension: usize,
    delay: usize,
}

pub struct DelayCoordinatesIter<'a, T: 'a> {
    dimension: usize,
    delay: usize,
    current: usize,
    slice: &'a [T],
    _phantom: PhantomData<&'a [T]>,
}

impl DelayCoordinates {
    pub fn new(dim: usize, delay: usize) -> Self {
        DelayCoordinates {
            dimension: dim,
            delay: delay,
        }
    }

    pub fn iter<'a, T>(&self, slice: &'a [T]) -> DelayCoordinatesIter<'a, T> {
        DelayCoordinatesIter {
            dimension: self.dimension,
            delay: self.delay,
            current: 0,
            slice: slice,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: Clone> Iterator for DelayCoordinatesIter<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let indices: Vec<usize> =
            (0..self.dimension).map(|x| self.current + x * self.delay).collect();
        if indices.iter().any(|&x| x >= self.slice.len()) {
            None
        } else {
            self.current += 1;
            Some(indices.iter().rev().map(|&x| self.slice[x].clone()).collect())
        }
    }
}
