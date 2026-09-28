use std::collections::HashMap;

pub fn mode(arr: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for &num in arr  {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut mode = 0;

    for (num, count) in &map{
        if *count > max_count{
            max_count = *count;
            mode = *num;
        }
    }

    mode
}