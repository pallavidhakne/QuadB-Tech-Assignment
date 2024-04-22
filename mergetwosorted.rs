fn merge_sorted_arrays(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut merged: Vec<i32> = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }

    if i < a.len() {
        merged.extend(&a[i..]);
    }

    if j < b.len() {
        merged.extend(&b[j..]);
    }

    merged
}

fn main() {
    let a = vec![1, 3, 5];
    let b = vec![2, 4, 6];
    let result = merge_sorted_arrays(a, b);
    println!("{:?}", result);
}
