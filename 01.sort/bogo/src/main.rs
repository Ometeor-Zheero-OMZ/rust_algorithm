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