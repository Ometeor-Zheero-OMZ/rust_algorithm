# Bogo Sort

```rust
use rand::Rng;
use rand::seq::SliceRandom;

/// 配列がソート済みかを判定する
///
/// windows(2) は隣り合う2つの要素をスライスとして扱う関数
fn is_sorted(nums: &[i32]) -> bool {
    nums.windows(2).all(|w| w[0] <= w[1])
}

/// bogoソート
///
/// シャッフルには SliceRandom トレイトを使用
fn bogo_sort(nums: &mut Vec<i32>) {
    let mut rng = rand::thread_rng();
    while !is_sorted(&nums) {
        nums.shuffle(&mut rng);
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (0..10) // 10個の数値を生成
        .map(|_| rng.gen_range(0..1000)) // 0から999の範囲から無作為に数値を生成
        .collect();

    println!("Before sorting: {:?}", nums);
    bogo_sort(&mut nums);
    println!("After sorting: {:?}", nums);
}
```

output

```bash
Before sorting: [250, 534, 230, 871, 298, 484, 67, 498, 356, 14]
After sorting:  [14, 67, 230, 250, 298, 356, 484, 498, 534, 871]
```

# Bubble Sort

```rust
use rand::Rng;

fn bubble_sort(nums: &mut Vec<i32>) {
    let len = nums.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.gen_range(0..1000))
        .collect();

    println!("Before sorting: {:?}", nums);
    bubble_sort(&mut nums);
    println!("After sorting: {:?}", nums);
}
```

output

```bash
Before sorting: [394, 189, 931, 711, 389, 536, 898, 495, 593, 715]
After sorting:  [189, 389, 394, 495, 536, 593, 711, 715, 898, 931]
```

# Cocktail Sort

```rust
use rand::Rng;

fn cocktail_sort(nums: &mut Vec<i32>) {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut swapped = true;

    while swapped {
        swapped = false;

        // 前方向に進むパス
        for i in start..end {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        end -= 1;
        swapped = false;

        // 後方向に進むパス
        for i in (start + 1..=end).rev() {
            if nums[i - 1] > nums[i] {
                nums.swap(i - 1, i);
                swapped = true;
            }
        }
        start += 1;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.gen_range(0..1000))
        .collect();

    println!("Before sorting: {:?}", nums);
    cocktail_sort(&mut nums);
    println!("After sorting:  {:?}", nums);
}
```

output

```bash
Before sorting: [132, 459, 52, 613, 664, 878, 965, 410, 189, 625]
After sorting:  [52, 132, 189, 410, 459, 613, 625, 664, 878, 965]
```

# Comb Sort

```rust
use rand::Rng;

fn comb_sort(nums: &mut Vec<i32>) {
    let mut gap = nums.len();
    let shrink_factor = 1.3;
    let mut swapped = true;

    while gap > 1 || swapped {
        gap = (gap as f64 / shrink_factor).floor() as usize;
        if gap < 1 {
            gap = 1;
        }

        swapped = false;

        for i in 0..nums.len() - gap {
            if nums[i] > nums[i + gap] {
                nums.swap(i, i + gap);
                swapped = true;
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.gen_range(0..1000))
        .collect();

    println!("Before sorting: {:?}", nums);
    comb_sort(&mut nums);
    println!("After sorting:  {:?}", nums);
}
```

output

```bash
Before sorting: [538, 120, 531, 285, 303, 319, 852, 795, 597, 388]
After sorting:  [120, 285, 303, 319, 388, 531, 538, 597, 795, 852]
```

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

# Selection Sort

```rust
use rand::Rng;

fn selection_sort(nums: &mut Vec<i32>) {
    let len = nums.len();

    for i in 0..len {
        let mut min_idx = i;

        for j in (i + 1)..len {
            if nums[j] < nums[min_idx] {
                min_idx = j;
            }
        }

        if i != min_idx {
            nums.swap(i, min_idx);
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.gen_range(0..1000))
        .collect();

    println!("Before sorting: {:?}", nums);
    selection_sort(&mut nums);
    println!("After sorting:  {:?}", nums);
}

```

output

```bash
Before sorting: [511, 821, 641, 905, 809, 428, 464, 279, 129, 364]
After sorting:  [129, 279, 364, 428, 464, 511, 641, 809, 821, 905]
```

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

# Bucket Sort

```rust
use rand::Rng;

fn insertion_sort(bucket: &mut Vec<i32>) {
    let len = bucket.len();
    for i in 1..len {
        let tmp = bucket[i]; // 現在の値を一時保存
        let mut j = i;     // 挿入位置を探索

        while j > 0 && bucket[j - 1] > tmp {
            bucket[j] = bucket[j - 1]; // 大きい要素を右にシフト
            j -= 1;
        }

        bucket[j] = tmp;
    }
}

fn bucket_sort(nums: &mut Vec<i32>) {
    let len = nums.len();
    if len == 0 {
        return; // 空のリストならそのまま終了
    }

    // 最大値と最長値を取得
    let max_val = *nums.iter().max().unwrap();
    let min_val = *nums.iter().min().unwrap();

    // バケットの数とサイズを決定
    let bucket_count = len; // バケットは配列のサイズと同じ
    let bucket_size = ((max_val - min_val) / bucket_count as i32) + 1;

    // バケットの初期化
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; bucket_count];

    // 要素を各バケットに分配
    for &num in nums.iter() {
        let idx = ((num - min_val) / bucket_size) as usize; // バケットインデックス
        buckets[idx].push(num); // 対応するバケットに追加
    }

    // 各バケットをソート
    for bucket in buckets.iter_mut() {
        insertion_sort(bucket);
    }

    // 結果を集約
    let mut sorted_nums: Vec<i32> = Vec::new();
    for bucket in buckets.iter() {
        sorted_nums.extend(bucket); // バケットを結合
    }

    // 元の配列に結果を反映
    nums.copy_from_slice(&sorted_nums);
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.gen_range(0..1000))
        .collect();

    println!("Before sorting: {:?}", nums);
    bucket_sort(&mut nums);
    println!("After sorting:  {:?}", nums);
}

```

output

```bash
Before sorting: [53, 419, 624, 416, 362, 651, 882, 737, 854, 168]
After sorting:  [53, 168, 362, 416, 419, 624, 651, 737, 854, 882]
```
