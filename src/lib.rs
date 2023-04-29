use rand::{thread_rng, Rng};
#[cfg(feature = "prefetch")]
use std::{
    arch::x86_64::{_mm_prefetch, _MM_HINT_T0},
    ptr,
};

pub struct Eytzinger(Vec<u32>);

impl From<&[u32]> for Eytzinger {
    fn from(input: &[u32]) -> Self {
        fn move_element(a: &[u32], b: &mut [u32], mut i: usize, k: usize) -> usize {
            if k <= a.len() {
                i = move_element(a, b, i, 2 * k);
                b[k] = a[i];
                i = move_element(a, b, i + 1, 2 * k + 1);
            }
            i
        }
        let mut result = Vec::with_capacity(input.len() + 1);
        result.resize(input.len() + 1, 0);
        move_element(&input[..], &mut result[..], 0, 1);
        Self(result)
    }
}

impl Eytzinger {
    /// Binary search over eytzinger layout array
    ///
    /// returns index of an element found or `0` is there is no match
    #[inline]
    pub fn binary_search(&self, value: u32) -> usize {
        let mut idx = 1;
        while idx < self.0.len() {
            #[cfg(feature = "prefetch")]
            unsafe {
                let prefetch = self.0.get_unchecked(2 * idx);
                _mm_prefetch::<_MM_HINT_T0>(ptr::addr_of!(prefetch) as *const i8);
            }

            idx = 2 * idx + usize::from(self.0[idx] < value);
        }
        idx >> (idx.trailing_ones() + 1)
    }

    pub fn as_slice(&self) -> &[u32] {
        &self.0
    }
}

pub fn generate_data(size: usize) -> Vec<u32> {
    let mut result = Vec::with_capacity(size);
    let delta = (u32::MAX / size as u32) + 1;
    let mut rng = thread_rng();
    let mut s = 0;
    while result.len() < size {
        s += rng.gen_range(1..delta);
        result.push(s);
    }

    result
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_eytzinger_create_simple() {
        let input = vec![0, 1, 2, 3, 4];
        let expected = vec![0, 3, 1, 4, 0, 2];
        let result = Eytzinger::from(&input[..]);
        assert_eq!(expected, result.as_slice());

        assert_eq!(1, result.binary_search(3));
    }

    #[test]
    fn check_eytzinger_search() {
        let input = vec![0, 1, 2, 3, 4];
        let eytz = Eytzinger::from(&input[..]);

        for i in input {
            assert!(eytz.binary_search(i) > 0);
        }
        assert!(eytz.binary_search(6) == 0)
    }

    #[test]
    fn check_eytzinger_functional() {
        let input = generate_data(1_000);
        let eytz = Eytzinger::from(&input[..]);

        for i in &input {
            let idx = eytz.binary_search(*i);
            let expected = eytz
                .as_slice()
                .iter()
                .enumerate()
                .find(|(_, el)| el == &i)
                .map(|(idx, _)| idx)
                .unwrap();
            assert_eq!(expected, idx);
        }
        assert!(eytz.binary_search(input.last().unwrap() + 1) == 0);
    }
}
