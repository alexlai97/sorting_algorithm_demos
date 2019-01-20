/// Sort by repeatedly steps through the list, compares adjacent pairs and swaps them if they are
/// in the wrong order.
///
/// # Example
/// To-be-sorted  array = ( 5, 1, 4, 2, 8 )
///
/// ## First Pass
/// ( **5 1** 4 2 8 ) → ( **1 5** 4 2 8 )  
/// ( 1 **5 4** 2 8 ) → ( 1 **4 5** 2 8 ) * Swap since 5 > 4  
/// ( 1 4 **5 2** 8 ) → ( 1 4 **2 5** 8 ) * Swap since 5 > 2  
/// ( 1 4 2 **5 8** ) → ( 1 4 2 **5 8** ) * Now, since these elements are already in order (8 > 5), algorithm does not swap them.  
///   
/// ## Second Pass  
/// ( **1 4** 2 5 8 ) → ( **1 4** 2 5 8 )  
/// ( 1 **4 2** 5 8 ) → ( 1 **2 4** 5 8 ) * Swap since 4 > 2  
/// ( 1 2 **4 5** 8 ) → ( 1 2 **4 5** 8 )  
/// ( 1 2 4 **5 8** ) → ( 1 2 4 **5 8** )   
///  
/// ## Third Pass  
/// the array is already sorted, but the algorithm does not know if it is completed. The algorithm needs one whole pass without any swap to know it is sorted.  
/// ( **1 2** 4 5 8 ) → ( **1 2** 4 5 8 )  
/// ( 1 **2 4** 5 8 ) → ( 1 **2 4** 5 8 )  
/// ( 1 2 **4 5** 8 ) → ( 1 2 **4 5** 8 )  
/// ( 1 2 4 **5 8** ) → ( 1 2 4 **5 8** )  
///
/// ```
/// # use sorting_algs_demos::bubblesort::bubblesort;
/// assert_eq!(bubblesort(vec![5, 1, 4, 2, 8]),
///     [1, 2, 4, 5, 8]);
/// ```
pub fn bubblesort(mut array: Vec<i32>) -> Vec<i32> {
    loop {
        let mut swapped: bool = false;
        for i in 0..array.len()-1 {
            if array[i] > array[i+1] {
                array.swap(i, i+1);
                swapped = true;
            }
        }
        if !swapped { break; }
    }
    array
}
