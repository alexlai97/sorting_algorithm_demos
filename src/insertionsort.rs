/// Sort by iterating through list, put the item in right position in front of current position
///
/// # Example
/// ```
/// # use sorting_algs_demos::insertionsort::insertionsort;
/// assert_eq!(insertionsort(vec![6, 5, 3, 7, 1, 8, 2, 4]),
///     vec![1, 2, 3, 4, 5, 6, 7, 8]);
/// ```
pub fn insertionsort(mut array: Vec<i32>) -> Vec<i32> {
    let len = array.len();
    for i in 1..len {
        for j in (0..i).rev() {
            if array[j+1] < array[j] {
                array.swap(j, j+1);
            }
        }
    }
    array
}
