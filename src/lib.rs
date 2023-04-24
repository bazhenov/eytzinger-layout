fn eytzinger(input: Vec<u32>) -> Vec<u32> {
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn foo() {
        let input = vec![0, 1, 2, 3, 4];
        let expected = vec![0, 3, 1, 4, 0, 2];
        let result = eytzinger(input);
        assert_eq!(expected, result);
    }
}
