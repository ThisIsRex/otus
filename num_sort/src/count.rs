pub fn count_sort(arr: &mut [u16]) {
    let mut table = vec![0; u16::MAX as usize + 1];

    arr.iter().for_each(|&num| {
        table[num as usize] += 1;
    });

    for i in 1..table.len() {
        table[i] += table[i - 1];
    }

    let mut temp = vec![0; arr.len()];

    arr.iter().rev().for_each(|&num| {
        let index = &mut table[num as usize];

        if *index == 0 {
            panic!("wtf?");
        }

        *index -= 1;

        temp[*index] = num;
    });

    arr.copy_from_slice(&temp);
}
