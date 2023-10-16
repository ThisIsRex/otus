pub trait InsertionSort<T, U, F>
where
    F: Fn(&T) -> U,
    U: Ord,
    T: std::fmt::Debug,
{
    fn insertion_sort(&mut self, getter: F);
}

impl<T, U, F> InsertionSort<T, U, F> for Vec<T>
where
    F: Fn(&T) -> U,
    U: Ord,
    T: std::fmt::Debug,
{
    fn insertion_sort(&mut self, getter: F) {
        let len = self.len();
        for i in 1..len {
            let mut j = i;
            while j > 0 && getter(&self[j - 1]) > getter(&self[j]) {
                self.swap(j, j - 1);
                j = j - 1;
            }
        }
    }
}
