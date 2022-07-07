fn main() {
    let mut v = vec!(4, 2, 9, 6, 3, 5, 10, 7, 1, 8);
    println!("Input: {:?}", v);
    merge_sort(&mut v);
    println!("Output {:?}", v);
}

fn merge_sort(nums: &mut Vec<i32>) {
    merge_sort_helper(nums, 0, nums.len()-1);
}

fn merge_sort_helper(nums: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end { return }

    let middle = ((end - start) / 2) + start;

    merge_sort_helper(nums, start, middle);
    merge_sort_helper(nums, middle+1, end);
    merge(nums, start, middle+1, end);
}

fn merge(nums: &mut Vec<i32>, start: usize, middle: usize, end: usize) {
    let (mut i, mut j) = (start, middle);
    let mut temp: Vec<i32> = Vec::new();

    loop {
        if i < middle && j <= end {
            if nums[i] <= nums[j] {
                temp.push(nums[i]);
                i += 1;
            } else {
                temp.push(nums[j]);
                j += 1;
            }

            continue;
        }

        break;
    }

    if i < middle {
        for x in i..middle {
            temp.push(nums[x]);
        }
    }

    if j <= end {
        for x in j..=end {
            temp.push(nums[x]);
        }
    }

    nums.splice(start..=end, temp);
}
