fn binary_search(nums: &Vec<i32>, target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return Some(mid);
        } else if nums[mid] < target {
            left = mid + 1; // 右側を探索
        } else {
            right = mid - 1; //左側を探索
        }
    }
    None
}

fn main() {
    let nums = vec![0, 1, 5, 7, 9, 11, 15, 20, 24];
    let target = 15;

    match binary_search(&nums, target) {
        Some(index) => println!("Index of element in list is: nums [{}]", index),
        None => println!("Element not found")
    }
}