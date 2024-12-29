pub fn linear_search(nums: &Vec<i32>, target: i32) -> Option<usize> {
    // イテレータは参照を返すため &num (参照の解除)
    for (i, &num) in nums.iter().enumerate() {
        if num == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let nums1 = vec![0, 1, 5, 7, 9, 11, 15, 20, 24];
    let target = 5;

    match linear_search(&nums1, target) {
        Some(index) => println!("Index of element in list is: nums[{}]", index),
        None => println!("Element not found in the list")
    }
}
