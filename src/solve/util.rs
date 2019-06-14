pub struct FisherShuffle {
    size: usize,
}

impl FisherShuffle {
    pub fn new(sz: usize) -> Self {
        FisherShuffle { size: sz }
    }

    pub fn array_to_num<T: Copy + Into<usize>>(&self, array: &[T]) -> usize {
        assert_eq!(array.len(), self.size);

        let mut memo = Vec::with_capacity(self.size);
        for i in 0..self.size {
            memo.push(i);
        }

        let mut res = 0;
        let mut b = 1;
        for (i, x) in array.iter().enumerate().rev() {
            let k = memo.iter().position(|&y| (*x).into() == y).unwrap();
            res += k * b;
            b *= i + 1;

            memo.swap(i, k);
        }

        res
    }

    pub fn num_to_array(&self, num: usize) -> Vec<usize> {
        let mut num: usize = num.into();

        let mut memo = Vec::with_capacity(self.size);
        for i in 0..self.size {
            memo.push(i);
        }

        for i in (0..self.size).rev() {
            let j = num % (i + 1);
            memo.swap(i, j);
            num /= i + 1;
        }
        memo
    }
}

#[test]
fn num_to_array_injective() {
    use std::collections::HashSet;

    const FACT8: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;
    let shuffle = FisherShuffle::new(8);

    let mut visited = HashSet::new();

    for i in 0..FACT8 {
        let array = shuffle.num_to_array(i);
        assert!(!visited.contains(&array));
        visited.insert(array);
    }
}

#[test]
fn num2array2num_identity() {
    const FACT8: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;
    let shuffle = FisherShuffle::new(8);

    for i in 0..FACT8 {
        let array = shuffle.num_to_array(i);
        let num = shuffle.array_to_num(&array);
        assert_eq!(num, i);
    }
}

pub struct VecU2 {
    vec: Vec<u8>,
    size: usize,
}

impl VecU2 {
    pub fn new(init: u8, sz: usize) -> Self {
        let mut init = init & 0x3;
        init |= init << 2;
        init |= init << 4;

        VecU2 {
            vec: vec![init; (sz + 3) / 4],
            size: sz,
        }
    }

    pub fn get(&self, idx: usize) -> u8 {
        assert!(idx < self.size);

        let (u8idx, u2idx) = (idx / 4, idx % 4);
        ((self.vec[u8idx] >> (u2idx * 2)) & 0x3) as u8
    }

    pub fn set(&mut self, idx: usize, val: u8) {
        assert!(idx < self.size);

        let (u8idx, u2idx) = (idx / 4, idx % 4);
        self.vec[u8idx] &= !(0x3 << (u2idx * 2));
        self.vec[u8idx] |= (val & 0x3) << (u2idx * 2);
    }
}
