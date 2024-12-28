# Insertion Sort

```rust
use rand::Rng;

fn insertion_sort(nums: &mut Vec<i32>) {
    let len = nums.len();

    for i in 1..len {
        let tmp = nums[i]; // 現在の要素を一時保存
        let mut j = i;   // 挿入位置を探索

        while j > 0 && nums[j - 1] > tmp {
            nums[j] = nums[j - 1]; // 大きい要素を右にシフト
            j -= 1; // 左へ移動
        }

        nums[j] = tmp;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.gen_range(0..1000))
        .collect();

    println!("Before sorting: {:?}", nums);
    insertion_sort(&mut nums);
    println!("After sorting:  {:?}", nums);
}

```

output

```bash
Before sorting: [476, 547, 408, 328, 310, 206, 84, 108, 584, 460]
After sorting:  [84, 108, 206, 310, 328, 408, 460, 476, 547, 584]
```
