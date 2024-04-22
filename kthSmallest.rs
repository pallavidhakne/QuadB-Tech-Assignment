fn kth_smallest(mut arr: Vec<i32>, k: usize) -> Option<i32> {
    if k == 0 || arr.len() < k {
        return None; 
    }

    arr.sort(); // Sort the array in non-decreasing order
    Some(arr[k - 1])
}
