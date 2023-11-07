use std::mem::size_of;

pub fn radix_sort(arr: &mut [u16]) {
    #[inline]
    fn get(num: u16, shift: usize) -> usize {
        ((num >> shift) & 0xff) as usize
    }

    let mut temp = vec![0; arr.len()];

    let mut rads_table = vec![0; u8::MAX as usize + 1];

    let mut shift = 0x00;

    const SHIFT_MAX: usize = size_of::<u16>() * 8;

    while shift < SHIFT_MAX {
        arr.iter().for_each(|&num| {
            let rad = get(num, shift);

            rads_table[rad] += 1;
        });

        for i in 1..rads_table.len() {
            rads_table[i] += rads_table[i - 1];
        }

        arr.iter().rev().for_each(|num| {
            let rad_index = get(*num, shift);

            let count = &mut rads_table[rad_index];

            if *count == 0 {
                return;
            }

            *count -= 1;

            temp[*count as usize] = *num;
        });

        arr.copy_from_slice(&temp);
        rads_table.fill(0);

        shift += size_of::<u8>() * 8;
    }
}
