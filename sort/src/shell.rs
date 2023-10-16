pub trait ShellSort<T, U, F>
where
    F: Fn(&T) -> U,
    U: Ord,
    T: std::fmt::Debug,
{
    fn shell_sort(&mut self, getter: F);
}

impl<T, U, F> ShellSort<T, U, F> for Vec<T>
where
    F: Fn(&T) -> U,
    U: Ord + PartialOrd,
    T: std::fmt::Debug,
{
    fn shell_sort(&mut self, getter: F) {
        let mut gap = self.len() / 2;

        while gap > 0 {
            for i in gap..self.len() {
                let mut j = i;

                while j >= gap && getter(&self[j - gap]) > getter(&self[j]) {
                    self.swap(j, j - gap);
                    j -= gap;
                }
            }

            gap /= 2;
        }
    }
}
