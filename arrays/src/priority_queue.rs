use crate::array::{ArrayOps, SingleArray};

type Item<T> = (T, u32);

fn find_index<T>(arr: &SingleArray<Item<T>>, target: u32) -> usize {
    let mut low = 0;
    let mut high = arr.size();

    while low < high {
        let mid = (low + high) / 2;
        if arr[mid].1 < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

#[derive(Debug)]
pub struct PriorityQueue<Item> {
    queue: SingleArray<Item>,
}

impl<T> PriorityQueue<Item<T>> {
    pub fn new() -> Self {
        Self {
            queue: SingleArray::new(crate::array::ExtendMode::Mul2),
        }
    }

    pub fn queue(&mut self, item: T, priority: u32) {
        let idx = find_index(&self.queue, priority);
        self.queue.add_at((item, priority), idx);
    }

    pub fn dequeue(&mut self) -> T {
        if self.queue.size() == 0 {
            panic!("Empty");
        }

        self.queue.remove(self.queue.size() - 1).0
    }
}

#[cfg(test)]
mod test {
    use super::PriorityQueue;

    #[test]
    fn test_it() {
        let mut q = PriorityQueue::new();
        q.queue(1, 1110);
        q.queue(2, 2);
        q.queue(3, 3);
        q.queue(4, 10);

        dbg!(&q);

        println!("{}", q.dequeue());
        println!("{}", q.dequeue());
        println!("{}", q.dequeue());
        println!("{}", q.dequeue());
    }
}
