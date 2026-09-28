pub fn median(arr: &mut [i32]) -> i32 {
    arr.sort();
    let middle = (arr.len())/2;
    arr[middle]
}