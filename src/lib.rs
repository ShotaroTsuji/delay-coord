use std::ops::Index;

/// Delay-coordinates
///
/// This trait describes delay-coordinates.
pub trait DelayCoordinates {
    /// Time delay of the delay-coordinates
    fn delay(&self) -> usize;
    /// Embedding dimension of the delay-coordinates
    fn dimension(&self) -> usize;
    /// Window size of the delay-coordinates
    fn window_size(&self) -> usize;
    /// Maps an index in the delay-coordinates into the index of the underlying series
    fn map_coord(&self, index: usize) -> Option<usize>;
}

/// Forward Delay-coordinates
///
/// Forward delay-coordinates is defined as for a series $x(t)$ as below:
///
/// $$
/// (x(t+(d-1)m), x(t+(d-2)m), \ldots, x(t+m), x(t)),
/// $$
///
/// where $m$ is the embedding dimension and $a$ is the time delay.
#[derive(Debug, Clone)]
pub struct ForwardDelayCoordinates {
    pub delay: usize,
    pub dimension: usize,
}

impl ForwardDelayCoordinates {
    /// Returns an iterator that produces `ForwardLiftedView`s
    pub fn mapping_iter<'a, T>(&'a self, slice: &'a [T]) -> ForwardMapping<'a, T, Self> {
        let ws = self.window_size();
        ForwardMapping {
            coord: &self,
            slice: slice,
        }
    }
}

impl DelayCoordinates for ForwardDelayCoordinates {
    #[inline]
    fn delay(&self) -> usize { self.delay }

    #[inline]
    fn dimension(&self) -> usize { self.dimension }

    /// Window size of the delay-coordinates
    #[inline]
    fn window_size(&self) -> usize {
        (self.dimension-1)*self.delay+1
    }

    /// Calculates the delay-coordinates
    #[inline]
    fn map_coord(&self, index: usize) -> Option<usize> {
        if index < self.dimension {
            Some((self.dimension-index-1)*self.delay)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForwardMapping<'a, T, C> {
    coord: &'a C,
    slice: &'a [T],
}

impl<'a, T, C> Iterator for ForwardMapping<'a, T, C>
where
    C: DelayCoordinates,
{
    type Item = DelayMappedView<'a, T, C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.len() < self.coord.window_size() {
            None
        } else {
            let view = DelayMappedView {
                coord: self.coord,
                slice: &self.slice[0..self.coord.window_size()],
            };
            self.slice = &self.slice[1..];
            Some(view)
        }
    }
}

/// View of a slice mapped in delay-coordinates
///
/// This struct provides an access for the underlying slice with indices in delay-coordinates.
#[derive(Debug, Clone)]
pub struct DelayMappedView<'a, T, C> {
    coord: &'a C,
    slice: &'a [T],
}

impl<'a, T, C> DelayMappedView<'a, T, C>
where
    C: DelayCoordinates,
{
    #[inline]
    pub fn get(&self, index: usize) -> Option<&'a T> {
        self.coord.map_coord(index).and_then(|pos| self.slice.get(pos))
    }

    #[inline]
    pub fn iter(&'a self) -> DelayMappedViewIter<'a, T, C> {
        DelayMappedViewIter {
            view: self,
            index: 0,
        }
    }
}

impl<'a, T, C> DelayMappedView<'a, T, C>
where
    T: Clone,
    C: DelayCoordinates,
{
    pub fn to_vec(&self) -> Vec<T> {
        (0..self.coord.dimension())
            .map(|index| self[index].clone())
            .collect()
    }
}

impl<'a, T, C> DelayMappedView<'a, T, C>
where
    T: std::iter::IntoIterator + Clone,
    C: DelayCoordinates,
{
    pub fn to_flatten_vec(&self) -> Vec<<T as std::iter::IntoIterator>::Item> {
        (0..self.coord.dimension())
            .map(|index| self[index].clone())
            .flatten()
            .collect()
    }
}

impl<'a, T, C> Index<usize> for DelayMappedView<'a, T, C>
where
    C: DelayCoordinates,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct DelayMappedViewIter<'a, T, C> {
    view: &'a DelayMappedView<'a, T, C>,
    index: usize ,
}

impl<'a, T, C> Iterator for DelayMappedViewIter<'a, T, C>
where
    C: DelayCoordinates,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.view.get(self.index);
        self.index += 1;
        item
    }
}

#[cfg(test)]
mod test {
    use crate::{DelayCoordinates, ForwardDelayCoordinates};

    #[test]
    fn test_forward_coord() {
        let data = (0..10).collect::<Vec<usize>>();
        let coord = ForwardDelayCoordinates {
            delay: 2,
            dimension: 3,
        };
        let mut iter = coord.mapping_iter(&data).map(|p| p.to_vec());
        assert_eq!(iter.next(), Some(vec![4, 2, 0]));
        assert_eq!(iter.next(), Some(vec![5, 3, 1]));
        assert_eq!(iter.next(), Some(vec![6, 4, 2]));
        assert_eq!(iter.next(), Some(vec![7, 5, 3]));
        assert_eq!(iter.next(), Some(vec![8, 6, 4]));
        assert_eq!(iter.next(), Some(vec![9, 7, 5]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_forward_coord_multi() {
        let data = (0..10).map(|n| vec![n, n]).collect::<Vec<_>>();
        let coord = ForwardDelayCoordinates {
            delay: 5,
            dimension: 2,
        };
        let mut iter = coord.mapping_iter(&data).map(|p| p.to_flatten_vec());
        assert_eq!(iter.next(), Some(vec![5, 5, 0, 0]));
        assert_eq!(iter.next(), Some(vec![6, 6, 1, 1]));
        assert_eq!(iter.next(), Some(vec![7, 7, 2, 2]));
        assert_eq!(iter.next(), Some(vec![8, 8, 3, 3]));
        assert_eq!(iter.next(), Some(vec![9, 9, 4, 4]));
        assert_eq!(iter.next(), None);
    }
}
