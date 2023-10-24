pub trait SelectionSort<T, U, F>
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug,
{
    fn selection_sort(&mut self, getter: F);
}

impl<T,U,F> SelectionSort<T, U, F> for Vec<T>
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug, {
    fn selection_sort(&mut self, getter: F) {
        let len = self.len();
        for i in 0..len {
            let mut smallest = i;
            
            for j in (i+1)..len {
                if getter(&self[j]) < getter(&self[smallest]) {
                    smallest = j;
                }
            }
            
            self.swap(i, smallest);
        }
    }
}