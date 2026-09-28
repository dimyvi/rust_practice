mod median;
mod mode;

fn main() {
    let mut v = vec![4, 4, 4, 692, 71, 15, 18, 9];

    let median_value = median::median(&mut v);
    let mode_value = mode::mode(&v);
    v.sort();

    println!("Median: {median_value}");
    println!("Mode: {mode_value}");
    println!("{:?}", v);
}
