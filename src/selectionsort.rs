/// Sort by iterating through the list, if smaller,replace the smallest in the rest with the current
///
/// # Example
/// ```
/// # use sorting_algs_demos::selectionsort::selectionsort;
/// assert_eq!(selectionsort(vec![6, 5, 4, 3, 2, 1]),
///     vec![1, 2, 3, 4, 5, 6]);
/// ```
pub fn selectionsort(mut array: Vec<i32>) -> Vec<i32> {
    let len = array.len();
    for i in 0..len-1 {
        // get the index smallest of smallest element in array[i..]
        let index_of_smallest = {
            let mut index = i;
            for j in i+1..len {
                if array[j] < array[index] {
                    index = j;
                }
            }
            index
        };
        if array[index_of_smallest] < array[i] {
            array.swap(index_of_smallest, i);
        }
    }
    array
}
