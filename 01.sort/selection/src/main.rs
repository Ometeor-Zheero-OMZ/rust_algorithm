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
