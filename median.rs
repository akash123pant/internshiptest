fn find_median(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n % 2 == 0 {
        // Even number of elements
        let mid_right = n / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] + arr[mid_right]) as f64 / 2.0
    } else {
        // Odd number of elements
        let mid = n / 2;
        arr[mid] as f64
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("Median: {}", find_median(&arr)); 

    let arr2 = [1, 2, 3, 4, 5, 6];
    println!("Median: {}", find_median(&arr2)); 
}
