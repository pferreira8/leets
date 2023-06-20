pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();
    for n in 0..nums.len() {
        let mut j = n + 1;
        //handle every number combination
        while j as usize <= nums.len() {
            if add_options(nums.get(n as usize), nums.get(j as usize)) == Some(target) {
                values.push(n as i32);
                values.push(j as i32);
            }
            j += 1;
        }
    }
    // list will always be even
    // since we add a pair every time
    assert_eq!((values.len() % 2), 0);
    return values
}

fn add_options(a: Option<&i32>, b: Option<&i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(*x + *y), // Add the inner values
        _ => None,                           // Handle cases where one or both options are None
    }
}