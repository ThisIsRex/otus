fn insert_into_bucket(arr: &mut Vec<u16>, value: u16) {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] < value {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    arr.insert(low, value);
}

pub fn bucket_sort(arr: &mut [u16]) {
    if arr.len() == 0 {
        return;
    }

    let max = *arr.iter().max().unwrap() as u64 + 1;

    let mut buckets = vec![vec![]; arr.len()];

    arr.iter().for_each(|&num| {
        let bucket_index = (num as u64) * (arr.len() as u64) / max;
        insert_into_bucket(&mut buckets[bucket_index as usize], num);
    });

    let mut arr_i = 0;
    buckets.iter().for_each(|bucket| {
        unsafe {
            let src_ptr = bucket.as_ptr();
            let dest_ptr = arr.as_mut_ptr().add(arr_i);

            std::ptr::copy(src_ptr, dest_ptr, bucket.len());
        }

        arr_i += bucket.len();
    });
}
