pub trait HeapSort<T, U, F>
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug,
{
    fn heap_sort(&mut self, getter: F);
}

fn heapify<T, U, F>(arr: &mut [T], getter: &F, len: usize, mut i: usize)
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug,
{
    loop {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < len && getter(&arr[left]) > getter(&arr[largest]) {
            largest = left;
        }

        if right < len && getter(&arr[right]) > getter(&arr[largest]) {
            largest = right;
        }

        if largest == i {
            break;
        }

        arr.swap(i, largest);
        i = largest;
    }
}

impl<T, U, F> HeapSort<T, U, F> for Vec<T>
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug,
{
    fn heap_sort(&mut self, getter: F) {
        let len = self.len();

        // построение максимальной кучи
        for i in (0..len / 2).rev() {
            heapify(self, &getter, len, i);
        }

        // извлечение элементов из кучи по одному
        for i in (0..len).rev() {
            self.swap(0, i);
            heapify(self, &getter, i, 0);
        }
    }
}
