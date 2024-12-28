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