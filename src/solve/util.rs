pub struct FisherShuffle {
    size: usize,
}

impl FisherShuffle {
    pub fn new(sz: usize) -> Self {
        FisherShuffle { size: sz }
    }

    pub fn array_to_num<T: Copy + Into<usize>>(&self, array: &[T]) -> usize {
        let mut memo = Vec::with_capacity(self.size);
        for i in 0..self.size {
            memo.push(i);
        }

        let mut pos = Vec::with_capacity(self.size);
        for i in 0..self.size {
            pos.push(i);
        }

        let mut res = 0;
        let mut b = 1;
        for i in (0..self.size).rev() {
            let x = array[i];
            let k = pos[x.into()];

            res += k * b;
            b *= i + 1;

            pos[memo[i]] = k;
            memo[k as usize] = memo[i];
        }

        res
    }

    pub fn num_to_array(&self, num: usize) -> Vec<usize> {
        let mut num: usize = num;

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

pub struct FisherShuffle8();

impl FisherShuffle8 {
    pub fn new() -> Self {
        FisherShuffle8()
    }

    pub fn array_to_num<T: Copy + Into<usize>>(&self, array: &[T]) -> usize {
        let mut memo = [0; 8];
        for (i, it) in memo.iter_mut().enumerate() {
            *it = i;
        }

        let mut pos = [0; 8];
        for (i, it) in pos.iter_mut().enumerate() {
            *it = i;
        }

        let mut res = 0;
        let mut b = 1;
        for i in (0..8).rev() {
            let x = array[i];
            let k = pos[x.into()];
            res += k * b;
            b *= i + 1;

            pos[memo[i]] = k;
            memo[k as usize] = memo[i];
        }

        res
    }

    pub fn num_to_array(&self, num: usize) -> Vec<usize> {
        let mut num: usize = num;

        let mut memo = Vec::with_capacity(8);
        for i in 0..8 {
            memo.push(i);
        }

        for i in (0..8).rev() {
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
    let shuffle = FisherShuffle8::new();

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
    let shuffle = FisherShuffle8::new();

    for i in 0..FACT8 {
        let array = shuffle.num_to_array(i);
        let num = shuffle.array_to_num(&array);
        assert_eq!(num, i);
    }
}

#[derive(Deserialize, Serialize)]
pub struct VecU2 {
    #[serde(with = "serde_bytes")]
    vec: Vec<u8>,
    size: u64,
}

impl VecU2 {
    pub fn new(init: u8, sz: usize) -> Self {
        let mut init = init & 3;
        init |= init << 2;
        init |= init << 4;

        VecU2 {
            vec: vec![init; (sz + 3) / 4],
            size: sz as u64,
        }
    }

    pub fn get(&self, idx: usize) -> u8 {
        debug_assert!(idx < self.size as usize);

        let (u8idx, u2idx) = (idx >> 2, idx & 3);
        ((self.vec[u8idx] >> (u2idx << 1)) & 3) as u8
    }

    pub fn set(&mut self, idx: usize, val: u8) {
        debug_assert!(idx < self.size as usize);

        let (u8idx, u2idx) = (idx >> 2, idx & 3);
        self.vec[u8idx] &= !(0x3 << (u2idx << 1));
        self.vec[u8idx] |= (val & 3) << (u2idx << 1);
    }
}
