use rand::{thread_rng, RngCore};
use std::collections::HashSet;
#[cfg(prefetch)]
use std::{
    arch::x86_64::{_mm_prefetch, _MM_HINT_T0},
    ptr,
};

pub fn eytzinger(input: &[u32]) -> Vec<u32> {
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
    result
}

pub fn generate_data(size: usize) -> Vec<u32> {
    let mut visited = HashSet::new();
    let mut result = Vec::with_capacity(size);
    let mut rng = thread_rng();
    while result.len() < size {
        let value = rng.next_u32();
        if visited.insert(value) {
            result.push(value);
        }
    }
    result.sort();

    result
}

pub fn eytzinger_binary_search(input: &[u32], value: u32) -> Result<usize, usize> {
    let mut idx = 1;
    while idx < input.len() {
        #[cfg(prefetch)]
        unsafe {
            let prefetch = input.get_unchecked(4 * idx);
            _mm_prefetch::<_MM_HINT_T0>(ptr::addr_of!(prefetch) as *const i8);
        }

        let el = input[idx];
        if el == value {
            return Ok(idx);
        }
        idx = 2 * idx + usize::from(el < value);
    }
    Err(0)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_eytzinger_create_simple() {
        let input = vec![0, 1, 2, 3, 4];
        let expected = vec![0, 3, 1, 4, 0, 2];
        let result = eytzinger(&input);
        assert_eq!(expected, result);

        assert_eq!(Ok(3), input.binary_search(&3));
    }

    #[test]
    fn check_eytzinger_search() {
        let input = vec![0, 1, 2, 3, 4];
        let eytz = eytzinger(&input);

        for i in input {
            assert!(eytzinger_binary_search(&eytz, i).is_ok());
        }
        assert!(eytzinger_binary_search(&eytz, 6).is_err())
    }

    #[test]
    fn check_eytzinger_functional() {
        let input = generate_data(1_000);
        let eytz = eytzinger(&input);

        for i in &input {
            let idx = eytzinger_binary_search(&eytz, *i).ok();
            let expected = eytz
                .iter()
                .enumerate()
                .find(|(_, el)| el == &i)
                .map(|(idx, _)| idx);
            assert_eq!(expected, idx);
        }
        assert!(eytzinger_binary_search(&eytz, input.last().unwrap() + 1).is_err());
    }
}
