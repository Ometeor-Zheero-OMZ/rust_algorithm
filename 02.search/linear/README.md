# 線形探索

```rust
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

    println!("============== Linear Search ==============");
    match linear_search(&nums1, target) {
        Some(index) => println!("Index of element in list is: nums[{}]", index),
        None => println!("Element not found in the list")
    }
}
```

```bash
Index of element in list is: nums[2]
```

## 要素の変更

vec 自体はインデックスを指定して、要素を変更するが、線形探索を使用した値の変更はどうするのか。

```rust
fn main() {
    let mut nums = vec![0, 1, 5, 7, 9, 11, 15, 20, 24];
    let index = 2;
    let new_val = 100;

    println!("List:         {:?}", nums);
    if index < nums.len() {
        nums[index] = new_val;
    }
    println!("Updated list: {:?}", nums);
}
```

```bash
List:         [0, 1, 5, 7, 9, 11, 15, 20, 24]
Updated list: [0, 1, 100, 7, 9, 11, 15, 20, 24]
```

## vec をループし、可変参照で要素を変更

もし指定した要素の値を変更する場合は、num を可変参照すること。
target が見つかった場合は、参照解除をして値を直接変更する。

```rust
fn update_element(nums: &mut Vec<i32>, target: i32, new_val: i32) {
    for num in nums.iter_mut() {
        if *num == target {
            *num = new_val;
        }
    }
}

fn main() {
    let mut nums3 = vec![0, 1, 5, 7, 9, 11, 15, 20, 24];
    let target = 5;
    let new_val = 100;

    println!("List:         {:?}", nums3);
    update_element(&mut nums3, target, new_val);
    println!("Updated list: {:?}", nums3);
}
```

```bash
List:         [0, 1, 5, 7, 9, 11, 15, 20, 24]
Updated list: [0, 1, 100, 7, 9, 11, 15, 20, 24]
```
