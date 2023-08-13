fn sb_lower_bound<T, Compare>(arr: &[T], value: &&T, comp: Compare) -> usize
where 
    Compare: Fn(&T, &T) -> bool, 
{
    let mut left = 0;
    let mut len = arr.len();

    while len > 0 {
        let half = len / 2;
        let mid = left + half;
        
        if comp(&arr[mid], value) {
            left = mid + 1;
            len -= half + 1;
        } else {
            len = half;
        }
    }
    left
}

fn compare_ints(x: &i32, y: &i32) -> bool {
    x < y
}

fn main() {
    let arr = [1, 2, 4, 4, 5, 6];
    let value = 4;
    let index = sb_lower_bound(&arr, &&value, compare_ints);
    println!("The lower bound of {} is at index {}", value, index);
}
