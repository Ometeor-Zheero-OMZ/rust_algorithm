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