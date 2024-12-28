# Gnome Sort

```rust
use rand::Rng;

fn gnome_sort(nums: &mut Vec<i32>) {
    let mut idx = 0;
    let len = nums.len();

    while idx < len {
        if idx == 0 || nums[idx] >= nums[idx - 1] {
            idx += 1; // 前方に進む
        } else {
            nums.swap(idx, idx - 1); // 後方に戻ってスワップ
            idx -= 1;
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.gen_range(0..1000))
        .collect();

    println!("Before sorting: {:?}", nums);
    gnome_sort(&mut nums);
    println!("After sorting:  {:?}", nums);
}

```

output

```bash
Before sorting: [901, 179, 198, 802, 20, 310, 232, 702, 77, 170]
After sorting:  [20, 77, 170, 179, 198, 232, 310, 702, 802, 901]
```