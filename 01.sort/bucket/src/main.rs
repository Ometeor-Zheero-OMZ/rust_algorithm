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
