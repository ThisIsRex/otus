pub trait MergeSort<T, U, F>
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug + Clone,
{
    fn merge_sort(&mut self, getter: F);
}

fn merge<T, U, F>(left: &[T], right: &[T], getter: &F) -> Vec<T>
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug + Clone,
{
    let mut merged: Vec<T> = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    let mut left_next = left_iter.next();
    let mut right_next = right_iter.next();

    while let (Some(l), Some(r)) = (left_next, right_next) {
        if getter(l) <= getter(r) {
            merged.push(l.clone());
            left_next = left_iter.next();
        } else {
            merged.push(r.clone());
            right_next = right_iter.next();
        }
    }

    merged.extend(left_next.cloned().into_iter().chain(left_iter.cloned()));
    merged.extend(right_next.cloned().into_iter().chain(right_iter.cloned()));

    merged
}

fn merge_sort<T, U, F>(arr: &mut [T], getter: &F)
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug + Clone,
{
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut arr[..mid], getter);
    merge_sort(&mut arr[mid..], getter);

    let mut merged = merge(&arr[..mid], &arr[mid..], getter);

    arr.swap_with_slice(&mut merged);
}

impl<T, U, F> MergeSort<T, U, F> for Vec<T>
where
    F: Fn(&T) -> U,
    U: PartialOrd,
    T: std::fmt::Debug + Clone,
{
    fn merge_sort(&mut self, getter: F) {
        merge_sort(self, &getter)
    }
}
