use rand::{thread_rng, RngCore};

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
    let mut result = Vec::with_capacity(size);
    let mut rng = thread_rng();
    let mut s = 0;
    for _ in 0..result.capacity() {
        let diff = rng.next_u32() % 15 + 1;
        s = s + diff;
        result.push(s);
    }

    result
}

pub fn eytzinger_binary_search(input: &[u32], value: u32) -> Result<usize, ()> {
    let input = &input[1..];
    let mut idx = 0;
    while idx < input.len() {
        let el = input[idx];
        if el == value {
            return Ok(idx);
        }
        idx = if value < el { 2 * idx + 1 } else { 2 * idx + 2 }
    }
    Err(())
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
            assert!(eytzinger_binary_search(&eytz, *i).is_ok());
        }
        assert!(eytzinger_binary_search(&eytz, input.last().unwrap() + 1).is_err());
    }
}
