/*
 * Quick sort:
 * - low memory usage
 * - cache friendly
 * - not stable
 * - best/average case O(n * log(n))
 * - worst case O(n^2)
 * - preferred for smaller data sets (which fit in memory)
 */

fn main() {
    let mut v = vec!(9, 2, 8, 5, 1, 6, 7, 3, 10, 4);
        println!("Input: {:?}", v);
        quick_sort(&mut v);
        println!("Output {:?}", v);
}

fn quick_sort(nums: &mut Vec<i32>) {
    quick_sort_helper(nums, 0, nums.len()-1);
}

fn quick_sort_helper(nums: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pi = partition(nums, low, high);
        quick_sort_helper(nums, low, pi-1);
        quick_sort_helper(nums, pi+1, high);
    }
}

fn partition(nums: &mut Vec<i32>, mut low: usize, high: usize) -> usize {
    let pivot = nums[high];

    for j in low..high {
        if nums[j] < pivot {
            nums.swap(low, j);
            low += 1;
        }
    }

    nums.swap(low, high);

    // Necessary due to our use of usize
    // Line 14's function's third argument will eventually be negative
    // without this if statement
    if low == 0 {
        return low+1;
    }

    low
}
