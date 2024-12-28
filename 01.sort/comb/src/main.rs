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