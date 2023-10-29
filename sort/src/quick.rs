pub trait QuickSort<T, U, F>
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug,
{
    fn quick_sort(&mut self, getter: F);
}

fn split<T, U, F>(arr: &mut [T], getter: &F) -> usize
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug,
{
    let m = arr.len() / 2;
    arr.swap(m, arr.len() - 1);

    let mut m = 0;

    for j in 0..arr.len() - 1 {
        if getter(&arr[j]) <= getter(&arr[arr.len() - 1]) {
            arr.swap(m, j);
            m += 1;
        }
    }

    arr.swap(m, arr.len() - 1);

    m
}

fn qs<T, U, F>(arr: &mut [T], getter: &F)
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug,
{
    if arr.len() <= 1 {
        return;
    }

    let m = split(arr, getter);
    qs(&mut arr[0..m], getter);
    qs(&mut arr[m + 1..], getter);
}

impl<T, U, F> QuickSort<T, U, F> for Vec<T>
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug,
{
    fn quick_sort(&mut self, getter: F) {
        qs(self, &getter);
    }
}
