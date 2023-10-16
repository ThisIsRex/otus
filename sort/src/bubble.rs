use std::ops::{Index, IndexMut};

pub trait BubbleSort<T, U, F>
where
    F: Fn(&T) -> U,
    U: Ord,
    T: std::fmt::Debug,
{
    fn bubble_sort(&mut self, getter: F);
}

impl<T, U, F> BubbleSort<T, U, F> for Vec<T>
where
    F: Fn(&T) -> U,
    U: Ord,
    T: std::fmt::Debug,
{
    fn bubble_sort(&mut self, getter: F) {
        let len = self.len();
        for i in 0..len {
            for j in 0..(len - i - 1) {
                unsafe {
                    if getter(&self.get_unchecked(j)) > getter(&self.get_unchecked(j + 1)) {
                        self.swap(j, j + 1);
                    }
                }
            }
        }
    }
}
