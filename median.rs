fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len == 0 {
        panic!("Array must not be empty");
    }

    if len % 2 == 1 {
       
        arr[len / 2] as f64
    } else {
        (arr[len / 2 - 1] as f64 + arr[len / 2] as f64) / 2.0
    }
}
