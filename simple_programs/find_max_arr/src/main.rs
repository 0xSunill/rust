fn main() {
    let arr = [1, 2, 5, 6, 62, 23, 35];
    let mut max = 0;
    for &arr in arr.iter() {
        if max < arr {
            max = arr;
        }
    }

    println!("the largest number is {}", max);
}
