use std::ops::{Index, IndexMut};

pub trait ArrayOps {
    type Item;

    fn size(&self) -> usize;

    fn add(&mut self, item: Self::Item);
    fn add_at(&mut self, item: Self::Item, i: usize);
    fn remove(&mut self, i: usize) -> Self::Item;
}

fn unsafe_copy<T>(src: &Vec<T>, src_idx: usize, dst: &mut Vec<T>, dst_idx: usize, len: usize) {
    assert!(src_idx + len <= src.len());
    assert!(dst_idx + len <= dst.len());

    unsafe {
        std::ptr::copy(
            src.as_ptr().add(src_idx),
            dst.as_mut_ptr().add(dst_idx),
            len,
        );
    }
}

fn shift_from_index<T>(vec: &mut Vec<T>, index: usize, offset: isize) {
    assert!(index < vec.len());

    let dst = if offset < 0 {
        index - ((-offset) as usize)
    } else {
        index + offset as usize
    };

    unsafe {
        let src_ptr = vec.as_ptr().add(index);
        let dst_ptr = vec.as_mut_ptr().add(dst);
        let len = vec.len() - index;

        std::ptr::copy(src_ptr, dst_ptr, len);
    }
}

// fn shift_left_from_index<T>(vec: &mut Vec<T>, index: usize) {
//     shift_from_index(vec, index, -1)
// }

fn shift_right_from_index<T>(vec: &mut Vec<T>, index: usize) {
    shift_from_index(vec, index, 1)
}

const INIT_LEN: usize = 1;

fn assert_bounds(index: usize, size: usize) {
    if index >= size {
        panic!("Index out of bounds!");
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExtendMode {
    Add(usize),
    Mul(usize),
    Mul2,
}

#[derive(Debug)]
pub struct SingleArray<T> {
    mem: Vec<T>,
    real_size: usize,
    extend_mode: ExtendMode,
}

impl<T> SingleArray<T> {
    pub fn new(extend_mode: ExtendMode) -> Self {
        let mut mem = Vec::with_capacity(INIT_LEN);

        unsafe {
            mem.set_len(INIT_LEN);
        }

        Self {
            mem,
            extend_mode,
            real_size: 0,
        }
    }

    fn extend_on_demand(&mut self) {
        if self.real_size == self.mem.len() {
            let new_size = match self.extend_mode {
                ExtendMode::Add(i) => self.mem.len() + i,
                ExtendMode::Mul(f) => self.mem.len() * f,
                ExtendMode::Mul2 => self.mem.len() * 2,
            };

            let mut new_mem = Vec::with_capacity(new_size);

            unsafe {
                new_mem.set_len(new_size);
            }

            unsafe_copy(&self.mem, 0, &mut new_mem, 0, self.real_size);

            self.mem = new_mem;
        }
    }
}

impl<T> ArrayOps for SingleArray<T> {
    type Item = T;

    fn size(&self) -> usize {
        self.real_size
    }

    fn add(&mut self, item: Self::Item) {
        self.extend_on_demand();

        let idx = self.size();

        self.mem[idx] = item;
        self.real_size += 1;
    }

    fn add_at(&mut self, item: Self::Item, i: usize) {
        self.extend_on_demand();

        shift_right_from_index(&mut self.mem, i);
        self.mem[i] = item;
        self.real_size += 1;
    }

    fn remove(&mut self, i: usize) -> Self::Item {
        self.real_size -= 1;
        // rust не позволяет просто так взять и получить владение элементом :((
        // тут всё должно быть по аналогии со вставкой
        //забираем элемент и сдвигаем хвост влево
        self.mem.remove(i)
    }
}

impl<T> Index<usize> for SingleArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert_bounds(index, self.size());

        &self.mem[index]
    }
}

impl<T> IndexMut<usize> for SingleArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert_bounds(index, self.size());

        &mut self.mem[index]
    }
}
