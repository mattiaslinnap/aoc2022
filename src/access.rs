pub fn independent_index<T>(s: &mut [T], idx1: usize, idx2: usize) -> (&mut T, &mut T) {
    assert!(idx1 < s.len());
    assert!(idx2 < s.len());
    assert_ne!(idx1, idx2);

    if idx1 < idx2 {
        let parts = s.split_at_mut(idx2);
        let first = &mut parts.0[idx1];
        let second = &mut parts.1[0];
        (first, second)
    } else {
        // idx1 > idx2
        let parts = s.split_at_mut(idx1);
        let first = &mut parts.1[0];
        let second = &mut parts.0[idx2];
        (first, second)
    }
}


#[cfg(test)]
mod tests {
    use crate::access::independent_index;

    #[test]
    #[should_panic]
    fn test_access_overflow() {
        let mut v = vec![1, 2, 3];
        independent_index(&mut v, 1, 5);
    }

    #[test]
    #[should_panic]
    fn test_access_same_index() {
        let mut v = vec![1, 2, 3];
        independent_index(&mut v, 1, 1);
    }

    #[test]
    fn test_access_diff() {
        let mut v = vec![1, 2, 3];
        let (a, b) = independent_index(&mut v, 1, 2);
        *a += 20;
        *b += 30;
        assert_eq!(v, vec![1, 22, 33]);
    }

    #[test]
    fn test_access_diff_reversed() {
        let mut v = vec![1, 2, 3];
        let (b, a) = independent_index(&mut v, 2, 1);
        *a += 20;
        *b += 30;
        assert_eq!(v, vec![1, 22, 33]);
    }
}
