fn sb_lower_bound<T, Compare>(mut first: usize, last: usize, value: &T, comp: Compare) -> usize
where
    Compare: Fn(&usize, &T) -> bool,
{
    let mut length = last - first;
    while length > 0 {
        let half = length / 2;
        let mid = first + half;
        // Check the value at the mid position
        if comp(&mid, value) {
            first = mid + 1;
            length -= half + 1;
        } else {
            length = half;
        }
    }
    first
}

fn main() {
    let nums = vec![1, 2, 4, 4, 6, 7];  // You can modify this list as needed
    let target_value = 4;               // You can modify this value as needed

    let position = sb_lower_bound(0, nums.len(), &target_value, |a: &usize, b: &i32| nums[*a] < *b);
    let expected_position = nums.iter().position(|&x| x == target_value).unwrap_or(nums.len());

    println!("Position of {} in the array: {}", target_value, position);
    println!("Expected position: {}", expected_position);
    
    if position == expected_position {
        println!("Test passed!");
    } else {
        println!("Test failed.");
    }
}
