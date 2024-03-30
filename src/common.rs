use crate::spatial::UPoint2D;

#[derive(Debug)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub width: usize,
}

impl<T> Matrix<T> {
    pub fn at(&self, at: UPoint2D) -> &'_ T {
        let idx = (at.row - 1) * self.width + (at.column - 1);
        &self.data[idx]
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.data.len() / self.width
    }
}

pub struct FiniteCycleIter<T> {
    pub values: Vec<T>,
    pub index: usize,
}

impl<T> Iterator for FiniteCycleIter<T>
where
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.values[self.index];
        self.index = (self.index + 1) % self.values.len();
        Some(current)
    }
}

#[derive(Debug)]
pub struct Cycle {
    pub mu: usize,     // the length of the prefix
    pub lambda: usize, // the actual cycle length
}

pub trait CycleDetection {
    fn detect_cycle(&self) -> Cycle;
}

impl<T> CycleDetection for T
where
    T: IntoIterator + Clone,
    T::Item: Eq,
{
    fn detect_cycle(&self) -> Cycle {
        let mut tortoise_iter = self.clone().into_iter();
        let mut hare_iter = self.clone().into_iter().skip(1);
        let mut tortoise = tortoise_iter.next().unwrap();
        let mut hare = hare_iter.next().unwrap();
        while tortoise != hare {
            tortoise = tortoise_iter.next().unwrap();
            hare_iter.next();
            hare = hare_iter.next().unwrap();
        }

        let mut mu = 0;
        tortoise_iter = self.clone().into_iter(); // reset stream
        tortoise = tortoise_iter.next().unwrap();
        hare = hare_iter.next().unwrap();
        while tortoise != hare {
            tortoise = tortoise_iter.next().unwrap();
            hare = hare_iter.next().unwrap();
            mu += 1;
        }

        let mut lambda = 1;
        hare = hare_iter.next().unwrap();
        while tortoise != hare {
            hare = hare_iter.next().unwrap();
            lambda += 1;
        }

        Cycle { mu, lambda }
    }
}
