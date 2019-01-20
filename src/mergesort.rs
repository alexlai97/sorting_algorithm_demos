pub fn mergesort(array: Vec<i32>) -> Vec<i32> {
    let len = array.len();

    if len == 1 { return array; }

    let mid = len / 2;

    let left = array[0..mid].to_vec();

    let right = array[mid..len].to_vec();

    let sorted_left = mergesort(left);

    let sorted_right = mergesort(right);

    merge(& sorted_left, & sorted_right)
}

fn merge(v_a: & [i32], v_b: & [i32]) -> Vec<i32> {
    let len_a = v_a.len();
    let len_b = v_b.len();
    let mut v: Vec<i32> = Vec::with_capacity(len_a + len_b);

    let mut i = 0;
    let mut j = 0;

    println!("A: {:?}, B: {:?}", v_a, v_b);
    while ! (i == len_a && j == len_b) {
        println!("i: {}, j: {}", i, j);
        if i < len_a && (j == len_b || v_a[i] <= v_b[j]) {
            v.push(v_a[i]);
            i += 1;
        } else {
            v.push(v_b[j]);
            j += 1;
        }
    }

    v
}


#[cfg(test)]    
mod tests {
    use super::*;

    #[test]
    fn merge_test() {
        let v_a = vec![1, 3, 5, 8, 10, 12];
        let v_b = vec![2, 4, 9];

        assert_eq!(merge(& v_a,& v_b), vec![1, 2, 3, 4, 5, 8, 9, 10, 12])
    }

    #[test]
    fn mergesort_test() {
        let v_a = vec![5, 2, 3, 6, 1, 8, 0, -4, 6, 7];
        assert_eq!(mergesort(v_a), vec![-4, 0, 1, 2, 3, 5, 6, 6, 7, 8]);
    }
} /* tests */
