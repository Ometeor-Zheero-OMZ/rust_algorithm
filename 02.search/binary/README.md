## バイナリサーチ

```rust
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
```

```bash
Index of element in list is: nums [6]
```

## 再帰的なバイナリサーチ

```rust

fn recur_binary_search(nums: &Vec<i32>, target: i32) -> Option<usize> {
    fn _binary_search(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> Option<usize> {
        if left > right {
            return None;
        }

        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return Some(mid);
        } else if nums[mid] < target {
            return _binary_search(nums, target, mid + 1, right); // 右半分を探索
        } else {
            return _binary_search(nums, target, left, mid - 1); // 左半分を探索
        }
    }

    _binary_search(nums, target, 0, nums.len() - 1)
}

fn main() {
    let nums = vec![0, 1, 5, 7, 9, 11, 15, 20, 24];
    let target = 15;

    match recur_binary_search(&nums, target) {
        Some(index) => println!("Index of element in list is: nums [{}]", index),
        None => println!("Element not found")
    }
}
```

```bash
Index of element in list is: nums [6]
```

## 要素の変更

```rust
fn update_element(nums: &mut Vec<i32>, target: i32, new_val: i32) {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            nums[mid] = new_val;
            return;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
}

fn main() {
    let mut nums = vec![0, 1, 5, 7, 9, 11, 15, 20, 24];
    let target = 15;
    let new_val = 100;

    println!("List:               {:?}", nums);
    update_element(&mut nums, target, new_val);
    println!("Updated list:       {:?}", nums);
}
```

```bash
List:               [0, 1, 5, 7, 9, 11, 15, 20, 24]
Updated list:       [0, 1, 5, 7, 9, 11, 100, 20, 24]
```
